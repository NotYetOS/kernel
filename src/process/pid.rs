use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;

struct PidAllocator {
    current: usize,
    recycled: Vec<usize>
}

impl PidAllocator {
    pub fn new() -> Self {
        Self {
            current: 0,
            recycled: Vec::new(),
        }
    }

    pub fn alloc(&mut self) -> Pid {
        match self.recycled.pop() {
            Some(value) => Pid(value),
            None => {
                self.current += 1;
                Pid(self.current - 1)
            }
        }
    }

    pub fn dealloc(&mut self, pid: usize) {
        self.recycled.push(pid)
    }
}

pub struct Pid(usize);

impl Pid {
    pub fn value(&self) -> usize {
        self.0
    }
}

impl Drop for Pid {
    fn drop(&mut self) {
        dealloc_pid(self.0)
    }
}

lazy_static! {
    static ref PID_ALLOCATOR: Mutex<PidAllocator> = {
        Mutex::new(PidAllocator::new())
    };
}

pub fn alloc_pid() -> Pid {
    PID_ALLOCATOR.lock().alloc()
}

pub fn dealloc_pid(pid: usize) {
    PID_ALLOCATOR.lock().dealloc(pid)
}
