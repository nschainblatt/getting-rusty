use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub mod modules {
    pub mod api;
    pub mod errors;
}

use modules::errors;

type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected, shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
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
    pub fn build(size: usize) -> Result<ThreadPool, errors::PoolCreationError> {
        if size < 1 {
            return Err(errors::PoolCreationError::InvalidSize);
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool {
            workers,
            sender: Some(sender),
        })
    }
    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        let job = Box::new(f);
        if let Some(sender) = &self.sender {
            sender.send(job).unwrap();
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
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
