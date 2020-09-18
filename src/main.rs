#![feature(async_closure)] //nightly mode feature
use async_std::sync::{Arc, Mutex};

mod handler;
mod server;
mod balancer;

type Connections = Arc<Mutex<Vec<async_std::net::TcpStream>>>;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    // Use shared vector of TcpStreams. Server adds streams, balancer send it to threads.
    // No reason to use queue since all requests are dispatched at once
    let streams: Connections = Arc::new(Mutex::new(Vec::new()));
    let streams1 = streams.clone();
    let streams2 = streams.clone();
    // Run balancer and server concurrently
    futures::join!(
        server::run(streams1),
        balancer::run(streams2, handler::handle_connection),
    );
    Ok(())
}