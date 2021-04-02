use super::unit::ProcessUnit;
use super::unit::TaskStatus;
use core::cell::RefCell;
use alloc::collections::VecDeque;
use lazy_static::lazy_static;
use spin::Mutex;
use alloc::sync::{
    Arc,
    Weak
};
use core::borrow::{
    Borrow, 
    BorrowMut
};

global_asm!(include_str!("process.s"));

extern "C" {
    fn _load(satp: usize); 
    fn _ret();
}

pub fn load(satp: usize) {
    unsafe {
        force_unlock_process_manager();
        _load(satp);
    }
}

pub fn ret() {
    unsafe { _ret() }
}

pub struct ProcessManager {
    process: VecDeque<Arc<ProcessUnit>>,
    current: RefCell<Option<Arc<ProcessUnit>>>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            process: VecDeque::new(),
            current: RefCell::new(None)
        }
    }

    pub fn push_process(
        &mut self, 
        process: Arc<ProcessUnit>
    ) {
        self.process.push_back(process);
    }

    pub fn run(&mut self) {
        loop { 
            if self.process.is_empty() { break; }
            self.run_next(); 
        }
    }

    pub fn exit(&mut self, code: i32) {
         match self.current.get_mut() {
            Some(process) => {
                process.set_zombie(code);
            }
            None => unreachable!()
        }
    }

    pub fn suspend(&mut self) {
        match self.current.get_mut() {
            Some(process) => {
                process.set_suspend();
            }
            None => unreachable!()
        }
    }

    pub fn pid(&mut self) -> usize {
        match self.current.get_mut() {
            Some(process) => {
                process.pid()
            }
            None => unreachable!()
        }
    }

    pub fn fork(&mut self) -> usize {
        let process = self.current();
        let child = process.fork();
        let pid = process.pid();
        self.push_process(child);
        self.current.replace(Some(process));
        pid
    }

    pub fn ready(&mut self) -> Option<usize> {
        match self.process.pop_front() {
            Some(mut process) => {
                let mut satp = process.satp();
                process.set_running();
                self.current = RefCell::new(Some(process));
                Some(satp)
            }
            None => None,
        }
    }

    pub fn run_next(&mut self) {
        match self.ready() {
            Some(satp) => load(satp),
            None => return
        }

        let process = self.current();

        match process.status() {
            TaskStatus::Zombie => {
                // RAII
                drop(process);
            }
            TaskStatus::Suspend => {
                self.push_process(process);
            }
            _ => unreachable!()
        }
    }
    
    fn current(&self) -> Arc<ProcessUnit> {
        self.current.replace(None).unwrap()
    }
}

lazy_static! {
    pub static ref PROCESS_MANAGER: Mutex<ProcessManager> = {
        Mutex::new(ProcessManager::new())
    };
}

pub fn run() {
    PROCESS_MANAGER.lock().run();
}

pub fn push_process(process: Arc<ProcessUnit>) {
    PROCESS_MANAGER.lock().push_process(process);
}

pub fn exit(code: i32) {
    PROCESS_MANAGER.lock().exit(code);
}

pub fn suspend() {
    PROCESS_MANAGER.lock().suspend();
}

pub fn getpid() -> usize {
    PROCESS_MANAGER.lock().pid()
}

pub fn fork() -> usize {
    PROCESS_MANAGER.lock().fork()
}

unsafe fn force_unlock_process_manager() {
    if PROCESS_MANAGER.is_locked() { 
        PROCESS_MANAGER.force_unlock() 
    };
}
