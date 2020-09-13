#![feature(async_closure)]

use async_std::io::*;
use async_std::prelude::*;

//mod taskpool;
mod open_file;
mod request;
//mod taskpool;

async fn handle_connection(mut stream: async_std::net::TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 4096];
    stream.read(&mut buffer).await?;
//    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let req = request::Request::new(&buffer).await;


//    let time = rand::random::<f32>() * 5.0;
//    println!("{}", time);
//    async_std::task::sleep(std::time::Duration::from_secs(time as u64)).await;
    let r = open_file::open_file(req.path()).await;
    if !r.is_ok() {
        stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n").await?;
    } else {
        let response = b"HTTP/1.1 200 OK\r\n\r\n";
        stream.write(b"HTTP/1.1 200 OK\r\n\r\n").await?;
        let mut w = async_std::io::BufWriter::new(stream);
        async_std::io::copy(&mut (r.unwrap()), &mut w).await?;
    }
    Ok(())
}

async fn server(streams: Arc<Mutex<Vec<async_std::net::TcpStream>>>) -> std::io::Result<()> {
    let listener = async_std::net::TcpListener::bind("127.0.0.1:3000").await?;
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        println!("New connection");
        let mut s = streams.lock().await;
        s.push(stream);
        drop(s);
        println!("disp");
    }

    Ok(())
}
async fn start_loop(streams: Arc<Mutex<Vec<async_std::net::TcpStream>>>) {
    let mut pool = TaskPool::new(4).await;
    let mut interval = async_std::stream::interval(std::time::Duration::from_millis(1));
    while let Some(_) = interval.next().await {
        let mut l = streams.lock().await;
        if l.len() > 0 {
            let mut new_streams: Vec<async_std::net::TcpStream> = Vec::new();
            for s in l.clone() {
                new_streams.push(s);
            }
            pool.execute(new_streams);

            l.clear();
        }
        drop(l);
    }
}
#[async_std::main]
async fn main() -> std::io::Result<()> {
    let streams: Arc<Mutex<Vec<async_std::net::TcpStream>>> = Arc::new(Mutex::new(Vec::new()));
    let mut streams1 = streams.clone();
    let mut streams2 = streams.clone();
    futures::join!(server(streams1), start_loop(streams2));
    Ok(())
}


use std::sync::mpsc;
use std::sync::Arc;
//use std::sync::Mutex;
use std::thread;
use async_std::sync::Mutex;

pub struct TaskPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Vec<async_std::net::TcpStream>;

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
//    pub async fn start_loop(&mut self) {
//        let mut interval = async_std::stream::interval(std::time::Duration::from_millis(1));
//        while let Some(_) = interval.next().await {
//            if self.tasks.len() > 0 {
//                self.sender.send(Message::NewJob(self.tasks.clone())).unwrap();
//                self.tasks.clear();
//            }
//        }
//    }
    pub async fn new(size: usize) -> TaskPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)).await);
        }

        TaskPool { workers, sender }
    }
    pub fn execute(&mut self, streams: Vec<async_std::net::TcpStream>)
    {
//        self.tasks.push(stream);
//        let job :Vec<async_std::net::TcpStream>= Vec::new();
//        self.sender.send(Message::NewJob(job)).unwrap();
        self.sender.send(Message::NewJob(streams)).unwrap();
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
                drop(task);//.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    task: Option<async_std::task::JoinHandle<()>>,
}

impl Worker {
    async fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let task = async_std::task::spawn((async move || loop {
            let message = receiver.lock().await.recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} received {} connections", id, job.len());
                    for task in job {
                        handle_connection(task).await;
                    }
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