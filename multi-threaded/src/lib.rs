use std::{sync::{mpsc, Arc, Mutex}, thread};

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>,
}

struct Job;

impl ThreadPool {
    pub fn new(num_threads: usize) -> Result<Self, &'static str> {
        if num_threads < 1 {
            return Err("number of threads must be at least 1");
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut threads = Vec::with_capacity(num_threads);
        for id in 0..num_threads {
            threads.push(Worker::new(id, receiver.clone()))
        }

        Ok(ThreadPool { threads, sender })
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {}
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let handle = thread::spawn(|| {
            receiver;
        });
        Worker {
            id,
            handle,
        }
    }
}
