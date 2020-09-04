use std::sync::mpsc::{channel, Receiver, RecvError};

/// A thread pool that wraps a `threadpool::ThreadPool`, but spawns "joinable" threads that behave
/// like those from `std::thread::spawn`.
///
/// # Example
///
/// ```
/// use joinable_threadpool::{ThreadPool, JoinHandle};
///
/// let pool = ThreadPool::new(4);
///
/// let handles: Vec<JoinHandle<String>> = (0..8)
///     .map(|i| pool.spawn(move || format!("Hello from thread {}", i)))
///     .collect();
///
/// for handle in handles {
///     let msg: String = handle.join().unwrap();
///     println!("Message: {}", msg);
/// }
///
/// ```
pub struct ThreadPool {
    inner: threadpool::ThreadPool,
}

impl ThreadPool {
    /// See `threadpool::ThreadPool::new`.
    ///
    /// # Panics
    ///
    /// This function will panic if `num_threads` is 0.
    pub fn new(num_threads: usize) -> Self {
        Self {
            inner: threadpool::ThreadPool::new(num_threads),
        }
    }

    /// Spawn a thread and return a handle which can be later `join`ed.
    pub fn spawn<F, T>(&self, f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = channel();

        self.inner.execute(move || tx.send(f()).unwrap());

        JoinHandle { rx }
    }
}

pub struct JoinHandle<T>
where
    T: Send + 'static,
{
    rx: Receiver<T>,
}

impl<T> JoinHandle<T>
where
    T: Send + 'static,
{
    /// Joins the thread and returns its value.
    pub fn join(self) -> Result<T, RecvError> {
        // Consume self to ensure rx.recv() is only called once
        self.rx.recv()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
