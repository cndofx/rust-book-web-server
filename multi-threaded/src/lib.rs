use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>,
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

        let mut threads = Vec::with_capacity(num_threads);
        for id in 0..num_threads {
            threads.push(Worker::new(id, receiver.clone()))
        }

        Ok(ThreadPool { threads, sender })
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let handle = thread::spawn(move || loop {
            let job = receiver
                .lock()
                .expect("unable to lock job receiver")
                .recv()
                .expect("unable to receive job");
            println!("worker {id} got a job, executing...");
            job();
            println!("worker {id} finished job");
        });
        Worker { id, handle }
    }
}
