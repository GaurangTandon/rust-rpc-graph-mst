use clap::{App, Arg};
use tarpc::{client, context, tokio_serde::formats::Json};
use std::{io, net::SocketAddr};

#[tokio::main]
async fn main() -> io::Result<()> {
    let app =
        App::new("Client Interactor")
            .version("1.0.0")
            .author("Gaurang Tandon")
            .about("client interface for rpc");

    let server_addr = "0.0.0.1:5001";
    let server_addr = server_addr
        .parse::<SocketAddr>().unwrap();

    let mut transport = tarpc::serde_transport::tcp::connect(server_addr, Json::default);
    transport.config_mut().max_frame_length(usize::MAX);

    let client = service::WorldClient::new(client::Config::default(), transport.await?).spawn()?;

    Ok(())
}