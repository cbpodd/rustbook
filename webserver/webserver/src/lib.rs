//! Library for the webserver.

use core::num::NonZeroUsize;
use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

type Job = Box<dyn FnOnce() + Send + 'static>;
type JobQueueSender = Sender<Job>;
type JobQueueReceiver = Arc<Mutex<Receiver<Job>>>;

/// A Thread Pool
#[derive(Debug)]
pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: JobQueueSender,
}

impl ThreadPool {
    /// Create a new [`ThreadPool`]
    pub fn new(size: NonZeroUsize) -> ThreadPool {
        let mut threads = Vec::with_capacity(size.get());

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for i in 0..size.get() {
            threads.push(Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool { threads, sender }
    }

    /// Execute a function on the threadpool
    #[allow(unused_results)] // Thread will be spawned and never returned.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).expect("Not handling errors.");
    }
}

#[derive(Debug)]
struct Worker {
    id: usize,
    handle: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: JobQueueReceiver) -> Self {
        let handle = thread::spawn(move || loop {
            let job = receiver
                .lock()
                .expect("Lock is poisoned. Panicking.")
                .recv()
                .expect("Not handling errors");

            // The lock is actually dropped here because any temporary values
            // (which the MutexGuard was) are dropped at the end of the let
            // statement.

            println!("Worker {id} got a job - executing.");

            job();
        });

        Worker { id, handle }
    }
}
