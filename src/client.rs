use clap::{App, Arg};
use tarpc::{client, context, tokio_serde::formats::Json};
use std::{io, net::SocketAddr, io::BufRead};

fn parse_int(n: &str) -> u32 {
    n.trim().parse().unwrap()
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let app =
        App::new("Client Interactor")
            .version("1.0.0")
            .author("Gaurang Tandon")
            .about("client interface for rpc");

    let server_addr = "0.0.0.0:5000";
    let server_addr = server_addr
        .parse::<SocketAddr>().unwrap();

    let mut transport = tarpc::serde_transport::tcp::connect(server_addr, Json::default);
    transport.config_mut().max_frame_length(usize::MAX);

    let mut client = service::GraphicalWorldClient::new(client::Config::default(), transport.await?).spawn()?;

    let stdin = io::stdin();
    loop {
        let mut inp = String::new();
        stdin.lock().read_line(&mut inp).unwrap();

        let tokens: Vec<&str> = inp.split(" ").collect();

        if tokens.is_empty() {
            println!("Invalid input: {}\n", inp);
            break;
        }

        let keyword = tokens[0];

        if keyword == "quit" {
            break;
        }

        let name = String::from(tokens[1].trim());

        if keyword == "clear" {
            client.clear_graph(context::current(), name).await?;
        } else if keyword == "add_graph" {
            let n: u32 = parse_int(tokens[2]);
            client.add_graph(context::current(), name, n).await?;
        } else if keyword == "add_edge" {
            let u: u32 = parse_int(tokens[2]);
            let v: u32 = parse_int(tokens[3]);
            let w: u32 = parse_int(tokens[4]);
            client.add_edge(context::current(), name, u, v, w).await?;
        } else if keyword == "get_mst" {
            client.get_mst(context::current(), name).await?;
        } else {
            println!("Invalid input: {}", inp);
            break;
        }
    }

    Ok(())
}