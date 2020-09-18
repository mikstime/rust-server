use async_std::prelude::*;
use async_std::sync::{Arc, Mutex};
type Connections = Arc<Mutex<Vec<async_std::net::TcpStream>>>;

//@TODO listen to port 80
pub async fn run(streams: Connections) -> std::io::Result<()> {
    // Listen to desired port
    let listener = async_std::net::TcpListener::bind("0.0.0.0:80").await?;
    // Get stream of incoming connections
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        // Lock vector of connections to push a new one
        let mut s = streams.lock().await;
        s.push(stream);
        // Unlock mutex
        drop(s);
    }

    Ok(())
}
