//! Library for the webserver.

use core::num::NonZeroUsize;

/// A Thread Pool
#[derive(Debug)]
pub struct ThreadPool;

impl ThreadPool {
    /// Create a new [`ThreadPool`]
    pub fn new(size: impl Into<NonZeroUsize>) -> ThreadPool {
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
