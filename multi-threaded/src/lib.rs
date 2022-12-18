pub struct ThreadPool {
    threads: Vec<std::thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(num_threads: usize) -> Result<Self, &'static str> {
        if num_threads < 1 {
            return Err("number of threads must be at least 1");
        } 
        let mut threads = Vec::with_capacity(num_threads);
        for _ in 0..num_threads {
            // create threads
        }

        Ok(ThreadPool { threads })
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {

    }
}