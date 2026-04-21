use std::thread;

pub struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            workers.push(thread::spawn(|| loop {}));
        }

        ThreadPool { workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        thread::spawn(f);
    }
}