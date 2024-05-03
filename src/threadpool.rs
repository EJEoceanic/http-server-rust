use std::{
    sync::{
        mpsc::{self, channel},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

pub struct Threadpool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl Threadpool {
    pub fn new(size: usize) -> Threadpool {
        assert!(size > 0);

        let (sender, receiver) = channel();

        let mut workers = Vec::with_capacity(4);

        let receiver = Arc::new(Mutex::new(receiver));

        for index in 0..size {
            workers.push(Worker::new(index, Arc::clone(&receiver)));
        }

        println!("Threadpool created with {size} workers");

        Threadpool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for Threadpool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap()
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // Change this to  std::thread::Builder at some point
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; Executing");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconected; Shutting down");
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
