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
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Mutex, Arc};

type Edge = (u32, u32, u32);
type GraphMap = HashMap<String, Vec<Edge>>;
type MutexLockedGraphMap = Arc<Mutex<GraphMap>>;

// This is the type that implements the generated World trait. It is the business logic
// and is used to start the server.
#[derive(Clone)]
struct HelloServer(SocketAddr, MutexLockedGraphMap);


#[tarpc::server]
impl GraphicalWorld for HelloServer {
    async fn add_graph(self, _: context::Context, name: String, n: u32) {
        let addr = self.0;
        println!("Received request to add graph {} of size {} from {:?}", name, n, addr);

        let edges: Vec<Edge> = vec![];

        let mut graphs = self.1.lock().unwrap();
        graphs.insert(name, edges);
    }

    async fn add_edge(self, _: context::Context, name: String, u: u32, v: u32, w: u32) {
        let addr = self.0;
        println!("Received request to add edge {},{},{} from {:?}", u, v, w, addr);

        let mut graphs = self.1.lock().unwrap();
        let edges = graphs.get_mut(&name);

        edges.expect("Non existent graph provided").push((u, v, w));
    }

    async fn get_mst(self, _: context::Context, name: String) -> i32 {
        let addr = self.0;
        0
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
