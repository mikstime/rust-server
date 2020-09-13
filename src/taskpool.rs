use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct TaskPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<async_std::net::TcpStream>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl TaskPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> TaskPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        TaskPool { workers, sender }
    }

    pub fn execute(&mut self, stream: async_std::net::TcpStream)
    {
        let job = Box::new(stream);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for TaskPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(task) = worker.task.take() {
                task;//.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    task: Option<async_std::task::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let task = async_std::task::spawn((async move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    println!("Handle");
                    handle_connection(*job).await;
//                    async_std::task::block_on(handle_connection(*job));
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        })());

        Worker {
            id,
            task: Some(task),
        }
    }
}

async fn handle_connection(mut stream: async_std::net::TcpStream) -> std::io::Result<()> {
    use async_std::io::*;
    use async_std::prelude::*;
    let mut buffer = [0; 4096];
    stream.read(&mut buffer).await?;
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    Ok(())
}