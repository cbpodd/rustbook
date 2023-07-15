//! Library for the webserver.

/// A Thread Pool
#[derive(Debug)]
pub struct ThreadPool;

impl ThreadPool {
    /// Create a new [`ThreadPool`]
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }

    /// Execute a function on the threadpool
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        f();
    }
}
