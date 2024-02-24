use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub mod modules {
    /// Handle incoming HTTP requests.
    pub mod api;
    /// Contains error Enums used throughout the library.
    pub mod errors;
}

use modules::errors;

type Job = Box<dyn FnOnce() + Send + 'static>;

/// A data structure to hold and run a thread assigned with an id.
///
/// `id` is a unique identifier to differentiate `Worker` types help in a `ThreadPool`.
///
/// `thread` is an `Option` holding the spawned thread. The `thread` field will hold a `None
/// variant when `ThreadPool` is dropped.
#[derive(Debug)]
pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Create a new `Worker` Result.
    ///
    /// `id` is a unique identifier.
    ///
    /// `receiver` is the receiver portion from the channel created in the `ThreadPool::Build`
    /// method. It receives a `Job` from the sender. The `Job` is a closure with trait `FnOnce()`
    /// which is given to the spawned thread to execute. Once the sender is closed an error will be
    /// sent to the receiver which will then break out of the thread loop and close out the thread.
    ///
    /// # errors
    /// Will panic if the 'Mutex' is in a poisoned state. Which is caused by another thread
    /// panicking while holding the `Mutex`.
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = if let Ok(message) = receiver.lock() {
                message.recv()
            } else {
                panic!("Another thread panicked while holding this mutex.");
            };
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

/// A pool of threads useful for many purposes, in this case it is used for handling incoming http
/// requests asynchronously.
///
/// `workers` is a vector of type `Worker`.
///
/// `sender` is an `Option` which holds a `Sender` from the channel created in the `Build`
/// method. It's job is to send `Job` types to the thread running on the `Worker`. Sender will hold
/// a None variant when the ThreadPool is dropped, this is to stop the thread loop and drop the
/// `Worker`.
#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new `ThreadPool` Result.
    ///
    /// `size` is the number of threads in the pool.
    ///
    /// # Errors
    ///
    /// The `build` function will return a `PoolCreationError` if creation of
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

    /// Send a `Job` to an availuable thread using the `Sender` from `ThreadPool`.
    ///
    /// `f` is a closure with trait bound `FnOnce()` to be sent as a `Job` to a thread.
    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        let job = Box::new(f);
        if let Some(sender) = &self.sender {
            match sender.send(job) {
                Ok(_) => (),
                Err(e) => panic!("Failed to send job to worker: {e}"),
            };
        } else {
            panic!("Sender not available");
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
