use clap::{App, Arg};
use futures::{future, prelude::*};
use service::GraphicalWorld;
use std::{
    io,
    net::{IpAddr, SocketAddr},
};
use tarpc::{
    context,
    server::{self, Channel, Handler},
    tokio_serde::formats::Json,
};
use std::collections::{HashMap, BTreeSet};
use std::hash::Hash;
use std::sync::{Mutex, Arc};

type Edge = (u32, u32, u32);
type EdgeSet = BTreeSet<Edge>;
type GraphData = (u32, EdgeSet);
type GraphMap = HashMap<String, GraphData>;
type MutexLockedGraphMap = Arc<Mutex<GraphMap>>;

// This is the type that implements the generated World trait. It is the business logic
// and is used to start the server.
#[derive(Clone)]
struct HelloServer(SocketAddr, MutexLockedGraphMap);

struct DSU {
    parents: Vec<usize>,
    sizes: Vec<usize>,
    n: usize,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        let parents: Vec<usize> = (0..=n).collect();
        let sizes = vec![1; n as usize + 1];

        Self { parents, sizes, n }
    }

    fn find(&mut self, node: usize) -> usize {
        if self.parents[node] != node {
            self.parents[node] = self.find(self.parents[node]);
        }

        return self.parents[node];
    }

    fn merge(&mut self, a: usize, b: usize) -> bool {
        let mut pa = self.find(a);
        let mut pb = self.find(b);

        if pa == pb {
            return false;
        }

        let sa = self.sizes[pa];
        let sb = self.sizes[pb];
        if sa < sb {
            let x = pa;
            pa = pb;
            pb = x;
        }

        self.sizes[pa] += self.sizes[pb];
        self.parents[pb] = pa;

        true
    }

    fn is_connected(&mut self) -> bool {
        let parent = self.find(1);

        for i in 2..=self.n {
            if self.find(i) != parent {
                return false;
            }
        }

        true
    }

    pub fn get_mst(&mut self, edges: &EdgeSet) -> i32 {
        let mut weight = 0;

        for (w, u, v) in edges {
            if self.merge(*u as usize, *v as usize) {
                weight += *w;
            }
        }

        if !self.is_connected() { -1 } else { weight as i32 }
    }
}

#[tarpc::server]
impl GraphicalWorld for HelloServer {
    async fn clear_graph(self, _: context::Context, name: String) {
        let addr = self.0;
        println!("Received request to clear graph {} from {:?}", name, addr);

        let mut graphs = self.1.lock().unwrap();
        if graphs.contains_key(&name) {
            graphs.remove(&name);
        }
    }

    async fn add_graph(self, _: context::Context, name: String, n: u32) {
        let addr = self.0;
        println!("Received request to add graph {} of size {} from {:?}", name, n, addr);

        let edges = BTreeSet::new();
        let data = (n, edges);

        let mut graphs = self.1.lock().unwrap();

        if graphs.contains_key(&name) {
            panic!("Graph {} already exists!", name);
        }

        graphs.insert(name, data);
    }

    async fn add_edge(self, _: context::Context, name: String, u: u32, v: u32, w: u32) {
        let addr = self.0;
        println!("Received request to add edge {},{},{} from {:?}", u, v, w, addr);

        if u == v {
            println!("Input has self-loop, skipping");
            return;
        }

        let mut graphs = self.1.lock().unwrap();
        let edges = graphs.get_mut(&name);
        edges.expect("Non existent graph cannot have edges inserted").1.insert((w, u, v));
    }

    async fn get_mst(self, _: context::Context, name: String) -> i32 {
        let addr = self.0;
        println!("Received request to find mst of {} from {:?}", name, addr);

        let mut edgeSet = (1, BTreeSet::new());

        {
            let mut graphs = self.1.lock().unwrap();
            let edges = graphs.get(&name).expect("Non existent graph does not have MST");
            edgeSet.0 = edges.0;
            edgeSet.1 = edges.1.clone();
        }

        let mut x = DSU::new(edgeSet.0 as usize);
        x.get_mst(&edgeSet.1)
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let app =
        App::new("RPC Server")
            .author("Gaurang Tandon")
            .version("1.0.0")
            .about("Server Interface for RPC")
            .arg(Arg::with_name("port").long("port").takes_value(true).required(true))
            .get_matches();

    let port = app.value_of("port").unwrap();
    let port = port.parse().unwrap();

    let mut graphs: GraphMap = HashMap::new();
    let mut graphs: MutexLockedGraphMap = Arc::new(Mutex::new(graphs));

    let server_addr = (IpAddr::from([0, 0, 0, 0]), port);
    // JSON transport is provided by the json_transport tarpc module. It makes it easy
    // to start up a serde-powered json serialization strategy over TCP.
    let mut listener = tarpc::serde_transport::tcp::listen(&server_addr, Json::default).await?;
    listener.config_mut().max_frame_length(4294967296);
    listener
        // Ignore accept errors.
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        // Limit channels to 1 per IP.
        .max_channels_per_key(1, |t| t.as_ref().peer_addr().unwrap().ip())
        // serve is generated by the service attribute. It takes as input any type implementing
        // the generated World trait.
        .map(|channel| {
            let addr = channel.as_ref().as_ref().peer_addr().unwrap();
            let graphs = Arc::clone(&graphs);
            let server = HelloServer(addr, graphs);
            channel.respond_with(server.serve()).execute()
        })
        // Max 10 channels.
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;

    Ok(())
}
