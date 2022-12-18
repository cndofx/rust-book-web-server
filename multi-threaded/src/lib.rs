use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

// struct Job;
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(num_threads: usize) -> Result<Self, &'static str> {
        if num_threads < 1 {
            return Err("number of threads must be at least 1");
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(num_threads);
        for id in 0..num_threads {
            workers.push(Worker::new(id, receiver.clone()))
        }

        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("shutting down worker {}", worker.id);
            if let Some(handle) = worker.handle.take() {
                handle.join().expect("unable to join worker thread handle");
            }
        }
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let handle = Some(thread::spawn(move || loop {
            let job = receiver
                .lock()
                .expect("unable to lock job receiver")
                .recv()
                .expect("unable to receive job");
            println!("worker {id} got a job, executing...");
            job();
            println!("worker {id} finished job");
        }));
        Worker { id, handle }
    }
}
