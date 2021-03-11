# Rust RPC

Uses the tarpc framework to have server/client framework in Rust

## Rust Installation

Single step:

`rustup` will install `rustc` (rust compiler), `cargo` (rust package manager), and `rustup` (rust toolchain manager). Get it from [here based on your system](https://www.rust-lang.org/tools/install). If it's Linux, this command should work: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`. If there's any warnings on this step, you can ignore them.

Now you should be able to run the project. There are no warnings at this step, it's my own source code.

### Running the server/client

0. `cd rust-rpc`
1. `cargo run --bin server -- <port>`
2. `cargo run --bin client -- <server_ip> <server_port> < <input_file>`

My server runs at `0.0.0.0`.

On the first run, it will download the necessary packages from the internet before compilation. That may take 5minutes depending on your connection speed. The compilation itself may take upto one minute to complete.

## Report

### Server

#### RPC framework

The TARPC framework requires a `trait` containing the signatures of the `async` RPC functions. This trait is marked by the 
attribute macro `#[tarpc:service]` which generates a corresponding trait for the Client to use.

The server then needs to `impl`ement this trait by providing method definitions, marking the corresponding implementation
with the attribute macro `#[tarpc:server]`.

TAPRC uses `tokio` which is the de-facto Rust library to handle network connections, and `serde` to provide JSON transport 
of data over network.

#### Mechanism 

1. Server initializes a TCP connection via `tarpc::serde_transport`, to entertain RPC requests from the client.
2. Server is waiting for incoming requests from clients via the connection
3. Server uses Kruskal to find the MST.
4. We use a `HashMap<String, Vec<(u32, u32, u32)>>` to store graphs key-ed by their names as a string. The vector stores 
edges as tuples of three elements u, v, w.
4. We use a `Arc<Mutex<>>` to make the hashmap threading consistent. The mutex ensures only one thread can attain read/write
access to the hashmap simultaneously. The `Arc` is an atomic reference counter that counts the number of threads maintaining
access to the mutex.
5. Everytime there is a client RPC request, we use the RPC framework to pass a struct containing the client address as well as
a copy of this `Arc` mutex into the trait implementation.

### Client

Client receives a `Client` trait of the corresponding service from tarpc framework. Client then needs to initiate a
connection to the server, and once done, can call RPC methods at will.

### Thread-safety

The code should be thread safe but I have not tested it yet. We were not given instructions on how to order concurrent
requests from several clients.