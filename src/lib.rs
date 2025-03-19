use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

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
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Message::Terminate => {
                    println!("Worker {id} was told to terminate.");
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

// enum Message {
//     NewJob(Job),
//     Terminate,
// }

pub struct ThreadPoolBuilder {
    size: Option<usize>,
    name: Option<String>,
}

impl ThreadPoolBuilder {
    pub fn new() -> Self {
        ThreadPoolBuilder {
            size: None,
            name: None,
        }
    }

    pub fn size(mut self, size: usize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn build(self) -> Result<ThreadPool, String> {
        let size = self.size.ok_or("Thread pool size not specified")?;
        
        if size == 0 {
            return Err("Thread pool size cannot be zero".to_string());
        }

        let pool_name = self.name.unwrap_or("ThreadPool".to_string());

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            if let Some(name) = &self.name {
                println!("Creating worker {} in pool '{}'", id, name);
            }
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }
}

// Add this to the ThreadPool impl block
impl ThreadPool {
    pub fn builder() -> ThreadPoolBuilder {
        ThreadPoolBuilder::new()
    }
}