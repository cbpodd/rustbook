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
    sender: Option<JobQueueSender>,
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

        ThreadPool {
            threads,
            sender: Some(sender),
        }
    }

    /// Execute a function on the threadpool
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender
            .as_ref()
            .expect("Not handling errors")
            .send(job)
            .expect("Not handling errors.");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.threads {
            println!("Shutting down worker {}", worker.id);

            if let Some(handle) = worker.handle.take() {
                handle.join().expect("Not handling errors");
            }
        }
    }
}

#[derive(Debug)]
struct Worker {
    id: usize,
    handle: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: JobQueueReceiver) -> Self {
        let handle = thread::spawn(move || 'receive_jobs: loop {
            let Ok(job) = receiver
                .lock()
                .expect("Lock is poisoned. Panicking.")
                .recv()
            else {
                println!("Sender was dropped. Worker {id} is exiting gracefully.");
                break 'receive_jobs;
            };

            // The lock is actually dropped here because any temporary values
            // (which the MutexGuard was) are dropped at the end of the let
            // statement.

            println!("Worker {id} got a job - executing.");
            job();
        });

        Worker {
            id,
            handle: Some(handle),
        }
    }
}
