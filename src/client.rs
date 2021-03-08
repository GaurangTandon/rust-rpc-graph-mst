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

    let server_addr = "0.0.0.0:5000";
    let server_addr = server_addr
        .parse::<SocketAddr>().unwrap();

    let mut transport = tarpc::serde_transport::tcp::connect(server_addr, Json::default);
    transport.config_mut().max_frame_length(usize::MAX);

    let mut client = service::WorldClient::new(client::Config::default(), transport.await?).spawn()?;

    // The client has an RPC method for each RPC defined in the annotated trait. It takes the same
    // args as defined, with the addition of a Context, which is always the first arg. The Context
    // specifies a deadline and trace information which can be helpful in debugging requests.
    let name = String::from("Hello");
    let hello = client.hello(context::current(), name).await?;

    println!("{}", hello);

    Ok(())
}