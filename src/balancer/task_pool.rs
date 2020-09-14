use async_std::prelude::*;
use async_std::sync::Arc;
use std::sync::mpsc;
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
    pub async fn new<F, Fut, T>(size: usize, handler: F) -> TaskPool
        where
            F: Fn(async_std::net::TcpStream)-> Fut + Send + Copy + 'static,
            T: Send,
            Fut: Future<Output = T> + Send + 'static,
    {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver), handler).await);
        }

        TaskPool { workers, sender }
    }
    pub fn execute(&mut self, streams: Vec<async_std::net::TcpStream>)
    {
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
    async fn new<F, Fut, T>(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>, handler: F) -> Worker
        where
            F: Fn(async_std::net::TcpStream)-> Fut + Send + 'static,
            T: Send,
            Fut: Future<Output = T> + Send + 'static,
    {
        let task = async_std::task::spawn((async move || loop {
            let message = receiver.lock().await.recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} received {} connections", id, job.len());
                    let mut tasks = Vec::new();
                    for task in job {
                        tasks.push(handler(task));
                    }
                    futures::future::join_all(tasks).await;
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
