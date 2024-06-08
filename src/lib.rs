use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

// This is a boxed (heap-allocated) trait object. It represents a function pointer or closure that:
// Can be called once (FnOnce())
// Can be sent between threads safely (Send)
// Has a 'static lifetime, meaning it doesn't contain any non-static (temporary) references.
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// # Arguments
    ///
    /// * `size`: The size is the number of threads in the pool.
    ///
    /// returns: ThreadPool
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // We’ll use a channel to function as the queue of jobs
        let (sender, receiver) = mpsc::channel();

        // The Arc type will let multiple workers own the receiver,
        // and Mutex will ensure that only one worker gets a job from the receiver at a time.
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(func);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Dropping sender closes the channel, which indicates no more messages will be sent.
        // When that happens, all the calls to recv that the workers do in the infinite loop will return an error
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down thread, id: {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // To share ownership of channel receiver across multiple threads and allow the threads to mutate the value,
    // we need to use Arc<Mutex<T>>.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // the move keyword is used to move the ownership of the receiver variable
        // into the closure that is passed to thread::spawn.
        let thread = thread::spawn(move || loop {
            // we first call lock on the receiver to acquire the mutex
            // if then we get the lock on the mutex, we call recv to receive a Job from the channel
            // this works with `let`, because any temporary values used in the expression
            // on the right hand side of the equals sign are immediately dropped when the let statement ends
            let job = receiver.lock().unwrap().recv();

            match job {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
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
