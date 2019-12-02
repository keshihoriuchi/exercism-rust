#![warn(clippy::all)]
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
    state: ThreadPoolState,
}

#[derive(Debug, PartialEq)]
pub enum ThreadPoolArgError {
    TooMinQueueSize,
    TooMinThreadSize,
    IllegalState,
}

#[derive(Debug, PartialEq)]
enum ThreadPoolState {
    Stop,
    Running,
}

impl ThreadPool {
    pub fn new(queue_size: usize, thread_size: usize) -> Result<ThreadPool, ThreadPoolArgError> {
        if queue_size < 1 {
            return Err(ThreadPoolArgError::TooMinQueueSize);
        }
        if thread_size < 1 {
            return Err(ThreadPoolArgError::TooMinThreadSize);
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(thread_size);
        for id in 0..thread_size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Ok(ThreadPool {
            workers,
            sender,
            state: ThreadPoolState::Stop,
        })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }

    pub fn start(&mut self) -> Result<(), ThreadPoolArgError> {
        if self.state == ThreadPoolState::Running {
            return Err(ThreadPoolArgError::IllegalState);
        }
        self.state = ThreadPoolState::Running;
        Ok(())
    }
    pub fn stop(&mut self) -> Result<(), ThreadPoolArgError> {
        if self.state == ThreadPoolState::Stop {
            return Err(ThreadPoolArgError::IllegalState);
        }
        self.state = ThreadPoolState::Stop;
        Ok(())
    }
}

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", id);

            job.call_box();
        });

        Worker { id, thread }
    }
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}
type Job = Box<dyn FnBox + Send + 'static>;

#[cfg(test)]
mod tests {
    use super::ThreadPool;
    use super::ThreadPoolArgError;
    use std::sync::{Arc, Condvar, Mutex};

    #[test]
    fn illegal_queue_size() {
        assert_eq!(
            ThreadPool::new(0, 1).unwrap_err(),
            ThreadPoolArgError::TooMinQueueSize
        );
    }

    #[test]
    fn illegal_thread_size() {
        assert_eq!(
            ThreadPool::new(1, 0).unwrap_err(),
            ThreadPoolArgError::TooMinThreadSize
        );
    }

    #[test]
    fn start_and_stop() {
        let mut tp = ThreadPool::new(1, 1).unwrap();
        tp.start().unwrap();
        tp.stop().unwrap();
    }

    #[test]
    fn stop_before_start() {
        let mut tp = ThreadPool::new(1, 1).unwrap();
        assert_eq!(tp.stop().unwrap_err(), ThreadPoolArgError::IllegalState);
    }

    #[test]
    fn restart_without_stop() {
        let mut tp = ThreadPool::new(1, 1).unwrap();
        tp.start().unwrap();
        assert_eq!(tp.start().unwrap_err(), ThreadPoolArgError::IllegalState);
    }

    // execute_before_start

    #[test]
    fn simple_execute() {
        let mut tp = ThreadPool::new(1, 1).unwrap();
        tp.start().unwrap();
        let pair = Arc::new((Mutex::new(0), Condvar::new()));
        let pair2 = pair.clone();
        tp.execute(move || {
            let (lock, cvar) = &*pair2;
            let mut count = lock.lock().unwrap();
            *count += 1;
            cvar.notify_one();
        });
        let (lock, cvar) = &*pair;
        let mut count = lock.lock().unwrap();
        while *count == 1 {
            count = cvar.wait(count).unwrap();
        }
        tp.stop().unwrap();
    }

    #[test]
    fn simple_repeated_dispatch() {
        let mut tp = ThreadPool::new(1, 1).unwrap();
        tp.start().unwrap();
        let pair = Arc::new((Mutex::new(0), Condvar::new()));
        for _ in 0..10 {
            let pair2 = pair.clone();
            tp.execute(move || {
                let (lock, cvar) = &*pair2;
                let mut count = lock.lock().unwrap();
                *count += 1;
                cvar.notify_all();
            });
        }
        let (lock, cvar) = &*pair;
        let mut count = lock.lock().unwrap();
        while *count == 10 {
            count = cvar.wait(count).unwrap();
        }
        tp.stop().unwrap();
    }

    #[test]
    fn complex_repeated_dispatch() {
        let mut tp = ThreadPool::new(10, 10).unwrap();
        tp.start().unwrap();
        let pair = Arc::new((Mutex::new(0), Condvar::new()));
        for _ in 0..1000 {
            let pair2 = pair.clone();
            tp.execute(move || {
                let (lock, cvar) = &*pair2;
                let mut count = lock.lock().unwrap();
                *count += 1;
                cvar.notify_all();
            });
        }
        let (lock, cvar) = &*pair;
        let mut count = lock.lock().unwrap();
        while *count == 1000 {
            count = cvar.wait(count).unwrap();
        }
        tp.stop().unwrap();
    }
}
