use std::thread;

pub struct ThreadPool {
    thread: Vec<thread::JoinHandle<()>>
}

impl ThreadPool {

    pub fn new(size: usize) -> Self {

        let mut threads = Vec::with_capacity(size);
        
        for _ in 0..usize {
            threads.push(|| {})
        }

        ThreadPool{ threads }
    }

}

pub struct Worker {
    id: u32,
    _thread: thread::JoinHandle<()>
}
