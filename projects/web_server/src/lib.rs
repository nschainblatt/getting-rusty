use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

struct Job;

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });
        Worker { id, thread }
    }
}

#[derive(Debug)]
pub enum PoolCreationError {
    InvalidSize,
}

#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool Result.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Errors
    ///
    /// The `build` function will return an error of type `PoolCreationError` if creation of
    /// ThreadPool fails.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size < 1 {
            return Err(PoolCreationError::InvalidSize);
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }
    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_and_user_thread_pool() {
        let pool = ThreadPool::build(4).unwrap();
        pool.execute(|| {
            println!("Hello from the new thread!");
        });
    }
}
