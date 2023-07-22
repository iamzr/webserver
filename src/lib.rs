use std::{fmt, 
    thread,
    sync::{mpsc, Arc, Mutex}
};

/// A struct representing a pool of threads 
pub struct ThreadPool{

    /// A vector of Workers that hold threads in this ThreadPool
    workers: Vec<Worker>,

    /// A channel sender that sends jobs to workers
    sender: Option<mpsc::Sender<Job>>
}

/// Type alias for a [`Job`] sent to [`Worker`]s of a [`ThreadPool`].
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Creates a new ThreadPool.
    ///
    /// The size is the number of threads in the pool
    /// 
    /// # Arguments
    /// 
    /// * `size` - The number of threads for this thread pool
    /// 
    /// # Errors
    ///
    /// This function will return an error if the size is zero.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError);
        }
        
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender: Some(sender)})
    }


    /// Executes the code inside the closure.
    /// It does this by creating a new job and then using the sender from the threadpool to send that job to next available worker. 
    ///
    /// # Panics
    ///
    /// Panics if unable to get the sender
    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    /// Returns the drop of this [`ThreadPool`].
    ///
    /// # Panics
    ///
    /// Panics if unable to join a thread in a worker to the current thread.
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

#[derive(Debug, Clone)]
/// Error for when there's an issue creating [`ThreadPool`].
pub struct PoolCreationError;

impl fmt::Display for PoolCreationError {
    /// Writes error [`PoolCreationError`].
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to create thread pool.")
    }
}

/// Worker for the [`ThreadPool`].
struct Worker {

    /// Id for this [`Worker`].
    id: usize,

    /// This [`Worker`]'s thread.
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Creates a new [`Worker`].
    ///
    /// # Panics
    ///
    /// Panics if unable to receive lock from the receiver.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing");

                    job();
                }

                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }
}