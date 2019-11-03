use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::JoinHandle;
use std::thread;

type Job =  FnOnce() + Send + 'static;

struct Worker {
    id:usize,
    thread_handler:JoinHandle<()>
}

impl Worker {
    fn new(_id:usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread_instance = thread::spawn( move||{
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                job();
            }
        });
        let worker = Worker {
            id:_id,
            thread_handler:thread_instance
        };
        return worker;
    }
}

struct ThreadPool {
    size:usize,
    workers:Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl ThreadPool {
    fn new(_size:usize) ->ThreadPool{
        let mut workerVec = Vec::with_capacity(_size);

        let (_sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0.._size {

            workerVec.push(Worker::new(id, Arc::clone(&receiver)));
        }

        return ThreadPool {
            size:_size,
            workers:workerVec,
            sender:_sender
        };
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }

}