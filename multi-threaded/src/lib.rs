use std::thread::JoinHandle;

pub struct ThreadPool {
    threads: Vec<Worker>,
}

struct Worker {
    id: usize,
    handle: JoinHandle<()>,
}

impl ThreadPool {
    pub fn new(num_threads: usize) -> Result<Self, &'static str> {
        if num_threads < 1 {
            return Err("number of threads must be at least 1");
        } 
        let mut threads = Vec::with_capacity(num_threads);
        for id in 0..num_threads {
            threads.push(Worker::new(id))
        }

        Ok(ThreadPool { threads })
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {

    }
}

impl Worker {
    fn new(id: usize) -> Self {
        Worker {
            id,
            handle: std::thread::spawn(|| {}),
        }
    }
}