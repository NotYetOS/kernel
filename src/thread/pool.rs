use core::ops::{Deref, DerefMut};

use crate::context::Context;
use crate::sbi::HSMHartStates;
use alloc::collections::BTreeMap;
use alloc::string::String;
use lazy_static::lazy_static;
use spin::{Mutex, MutexGuard};

lazy_static! {
    pub static ref THREAD_POOL: ThreadPool = ThreadPool::new();
}

pub struct ThreadPool {
    pool_size: usize,
    inner: Mutex<ThreadPoolInner>,
}

impl ThreadPool {
    pub fn new() -> Self {
        Self {
            pool_size: super::NUM_HART,
            inner: Mutex::new(ThreadPoolInner::new()),
        }
    }

    pub fn pool_size(&self) -> usize {
        self.pool_size
    }

    pub fn num_thread(&self) -> usize {
        self.lock().threads.len()
    }
}

pub struct ThreadPoolInner {
    threads: BTreeMap<usize, Thread>,
}

impl ThreadPoolInner {
    fn new() -> Self {
        Self {
            threads: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, thread: Thread) {
        self.threads.insert(thread.hart_id, thread);
    }
}

pub struct Thread {
    pub hart_id: usize,
    pub name: String,
    pub state: HSMHartStates,
    pub context: &'static Context,
}

impl Deref for ThreadPool {
    type Target = Mutex<ThreadPoolInner>;

    fn deref<'a>(&self) -> &Self::Target {
        &self.inner
    }
}
