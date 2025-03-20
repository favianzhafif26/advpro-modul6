use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn build() -> ThreadPoolBuilder {
        ThreadPoolBuilder {
            size: None,
            name_prefix: None,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

pub struct ThreadPoolBuilder {
    size: Option<usize>,
    name_prefix: Option<String>,
}

impl ThreadPoolBuilder {
    pub fn size(mut self, size: usize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn name_prefix(mut self, prefix: &str) -> Self {
        self.name_prefix = Some(prefix.to_string());
        self
    }

    pub fn create(self) -> ThreadPool {
        let size = self.size.unwrap_or(4);
        assert!(size > 0, "ThreadPool size must be greater than 0");

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let worker_name = match &self.name_prefix {
                Some(prefix) => format!("{}-{}", prefix, id),
                None => format!("worker-{}", id),
            };

            workers.push(Worker::new(id, Arc::clone(&receiver), worker_name));
        }

        ThreadPool { workers, sender }
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>, name: String) -> Worker {
        let thread = thread::Builder::new()
            .name(name)
            .spawn(move || {
                while let Ok(job) = receiver.lock().unwrap().recv() {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
            })
            .unwrap();

        Worker { id, thread }
    }
}