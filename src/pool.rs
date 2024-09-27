use std::{
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread,
};

type Job = Box<dyn FnOnce() -> () + Send + 'static>;

#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Option<Job>>,
}

#[derive(Debug)]
struct Worker {
    id: u32,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: u32, recevier: Arc<Mutex<mpsc::Receiver<Option<Job>>>>) -> Self {
        let thread: thread::JoinHandle<()> = thread::spawn(move || loop {
            if let Some(job) = recevier.lock().unwrap().recv().unwrap() {
                job();
            } else {
                break;
            }
        });
        Self { id, thread }
    }
    fn join(self) {
        self.thread.join().unwrap();
    }
}

impl ThreadPool {
    pub fn new(worker_count: u32) -> Self {
        let (sender, recevier) = mpsc::channel();
        let recevier = Arc::new(Mutex::new(recevier));

        let workers: Vec<_> = (0..worker_count)
            .map(|id| Worker::new(id, Arc::clone(&recevier)))
            .collect();

        Self { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() -> () + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Some(job)).unwrap();
    }

    pub fn wait(self) {
        for _ in 0..self.workers.len() {
            self.sender.send(None).unwrap();
        }
        for worker in self.workers {
            worker.join();
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use thread::sleep;

    use super::*;

    #[test]
    fn test_pool() {
        let pool = ThreadPool::new(8);
        for i in 0..16 {
            pool.execute(move || {});
        }
        pool.wait();
    }
}