use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(func);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
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
            // however, `while let` (and `if let` and `match`) does not drop temporary values until the end of the associated block.
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            // calling the given function
            job();
        });

        Worker { id, thread }
    }
}
