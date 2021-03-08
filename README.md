# Rust RPC

Uses the tarpc framework to have server/actor framework in Rust

## Installation

Only two steps:

`rustup` will install `rustc` (rust compiler), `cargo` (rust package manager), and `rustup` (rust toolchain manager). Get it from [here based on your system](https://www.rust-lang.org/tools/install). If it's Linux, this command should work: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`. If there's any warnings on this step, you can ignore them.

Now you should be able to run `cargo run rust-rpc`. There are no warnings at this step, it's my own source code.
    - this will download the necessary packages from the internet before compilation

The build will take some time (roughly 2mins)
