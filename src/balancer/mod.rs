use async_std::prelude::*;
use async_std::sync::{Arc, Mutex};

mod task_pool;

type Connections = Arc<Mutex<Vec<async_std::net::TcpStream>>>;

pub async fn run<F, Fut, T>(streams: Connections, handler: F)
    where
        F: Fn(async_std::net::TcpStream) -> Fut + Send + Copy + 'static,
        T: Send,
        Fut: Future<Output=T> + Send + 'static,
{
    // One worker for each CPU core
    let mut pool = task_pool::TaskPool::new(num_cpus::get(), handler).await;
    // Check each 4 millisecond for new connections. Process all in single task if found.
    let mut interval = async_std::stream::interval(std::time::Duration::from_millis(4));

    while let Some(_) = interval.next().await {
        // Lock streams to get length and and process connections if needed
        let mut locked_streams = streams.lock().await;
        if locked_streams.len() > 0 {
            let mut new_streams: Vec<async_std::net::TcpStream> = Vec::new();
            while locked_streams.len() > 0 && new_streams.len() < 24 {
                new_streams.push(locked_streams.pop().unwrap());
            }
            pool.execute(new_streams);
            if locked_streams.len() > 500 {
                async_std::task::sleep(std::time::Duration::from_millis(4)).await;
            }
        }
        // Unlock streams
        drop(locked_streams);
    }
}