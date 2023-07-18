use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

pub struct Channel<T> {
    buffer: Mutex<VecDeque<T>>,
    cvar: Condvar,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Channel {
            buffer: Mutex::new(VecDeque::new()),
            cvar: Condvar::new(),
        }
    }

    pub fn send(&self, t: T) {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.push_back(t);
        self.cvar.notify_one();
    }

    pub fn recv(&self) -> T {
        let mut buffer = self.buffer.lock().unwrap();
        while buffer.is_empty() {
            buffer = self.cvar.wait(buffer).unwrap();
        }
        buffer.pop_front().unwrap()
    }

    pub fn read_buffer<F, R>(&self, func: F) -> R
    where
        F: Fn(&VecDeque<T>) -> R,
    {
        let buffer = self.buffer.lock().unwrap();
        func(&buffer)
    }
}
