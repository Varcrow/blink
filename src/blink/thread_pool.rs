use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

pub struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    sender: mpsc::SyncSender<Box<dyn FnOnce() + Send + 'static>>,
}

impl ThreadPool {
    pub fn new(size: usize, queue_size: usize) -> Self {
        let (sender, receiver) = mpsc::sync_channel::<Box<dyn FnOnce() + Send>>(queue_size);
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            let r = Arc::clone(&receiver);
            workers.push(thread::spawn(move || {
                while let Ok(job) = r.lock().unwrap().recv() {
                    job();
                }
            }));
        }

        ThreadPool { workers, sender }
    }

    // Try to submit a job, returns Err if queue is full
    pub fn try_execute<F>(&self, f: F) -> Result<(), mpsc::TrySendError<Box<dyn FnOnce() + Send>>>
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.try_send(Box::new(f))
    }
}
