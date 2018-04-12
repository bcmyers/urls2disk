use std::sync::{Condvar, Mutex};

use num_cpus;

#[derive(Debug)]
pub(crate) struct Semaphore {
    max_requests_per_second: usize,
    max_threads_cpu: usize,
    max_threads_io: usize,

    requests: Mutex<usize>,
    requests_condvar: Condvar,
    threads_cpu: Mutex<usize>,
    threads_cpu_condvar: Condvar,
    threads_io: Mutex<usize>,
    threads_io_condvar: Condvar,
}

impl Default for Semaphore {
    fn default() -> Self {
        Semaphore {
            max_requests_per_second: 10,
            max_threads_cpu: num_cpus::get(),
            max_threads_io: 100,

            requests: Mutex::new(0),
            requests_condvar: Condvar::new(),
            threads_cpu: Mutex::new(0),
            threads_cpu_condvar: Condvar::new(),
            threads_io: Mutex::new(0),
            threads_io_condvar: Condvar::new(),
        }
    }
}

impl Semaphore {
    pub(crate) fn new(
        max_requests_per_second: usize,
        max_threads_cpu: usize,
        max_threads_io: usize,
    ) -> Self {
        let mut semaphore = Self::default();
        semaphore.max_requests_per_second = max_requests_per_second;
        semaphore.max_threads_cpu = max_threads_cpu;
        semaphore.max_threads_io = max_threads_io;
        semaphore
    }
    pub(crate) fn reset_requests(&self) {
        let mut requests = self.requests.lock().unwrap();
        *requests = 0;
        self.requests_condvar.notify_one();
    }
    pub(crate) fn increment_requests(&self) {
        let mut requests = self.requests.lock().unwrap();
        while *requests >= self.max_requests_per_second {
            requests = self.requests_condvar.wait(requests).unwrap()
        }
        *requests += 1;
        self.requests_condvar.notify_one();
    }
    pub(crate) fn increment_threads_cpu(&self) {
        let mut threads_cpu = self.threads_cpu.lock().unwrap();
        while *threads_cpu >= self.max_threads_cpu {
            threads_cpu = self.threads_cpu_condvar.wait(threads_cpu).unwrap()
        }
        *threads_cpu += 1;
        self.threads_cpu_condvar.notify_one();
    }
    pub(crate) fn decrement_threads_cpu(&self) {
        let mut threads_cpu = self.threads_cpu.lock().unwrap();
        *threads_cpu -= 1;
        self.threads_cpu_condvar.notify_one();
    }
    pub(crate) fn increment_threads_io(&self) {
        let mut threads_io = self.threads_io.lock().unwrap();
        while *threads_io >= self.max_threads_io {
            threads_io = self.threads_io_condvar.wait(threads_io).unwrap()
        }
        *threads_io += 1;
        self.threads_io_condvar.notify_one();
    }
    pub(crate) fn decrement_threads_io(&self) {
        let mut threads_io = self.threads_io.lock().unwrap();
        *threads_io -= 1;
        self.threads_io_condvar.notify_one();
    }
}
