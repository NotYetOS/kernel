use super::unit::ProcessUnit;
use super::unit::TaskStatus;
use core::cell::RefCell;
use alloc::collections::VecDeque;
use lazy_static::lazy_static;
use spin::Mutex;
use core::borrow::{
    Borrow, 
    BorrowMut
};

global_asm!(include_str!("process.s"));

extern "C" {
    fn _load(satp: usize); 
    fn _return();
}

pub struct ProcessManager {
    process: VecDeque<ProcessUnit>,
    current: RefCell<Option<ProcessUnit>>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            process: VecDeque::new(),
            current: RefCell::new(None)
        }
    }

    pub fn push_process(&mut self, process: ProcessUnit) {
        self.process.push_front(process);
    }

    pub fn run(&mut self) {
        loop { 
            if self.process.is_empty() { break; }
            self.run_next(); 
        }
    }

    pub fn exit(&mut self) {
         match self.current.get_mut() {
            Some(process) => {
                process.set_zombie()
            }
            None => unreachable!()
        }
        unsafe { _return(); }
    }

    pub fn suspend(&mut self) {
        match self.current.get_mut() {
            Some(process) => {
                process.set_suspend();
            }
            None => unreachable!()
        }
        unsafe { _return(); }
    }

    pub fn run_next(&mut self) {
        match self.process.pop_front() {
            Some(mut process) => {
                let task_unit = process.task_unit();
                let satp = task_unit.satp;
                process.set_running();
                self.current = RefCell::new(Some(process));
                unsafe { _load(satp); }
            }
            None => return,
        }

        let process = self.current.replace(None).unwrap();
    
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
}

lazy_static! {
    pub static ref PROCESS_MANAGER: Mutex<ProcessManager> = {
        Mutex::new(ProcessManager::new())
    };
}

pub fn run() {
    PROCESS_MANAGER.lock().run();
}

pub fn push_process(process: ProcessUnit) {
    PROCESS_MANAGER.lock().push_process(process);
}

pub fn exit() {
    unsafe { force_unlock_process_manager(); }
    PROCESS_MANAGER.lock().exit();
}

pub fn suspend() {
    unsafe { force_unlock_process_manager(); }
    PROCESS_MANAGER.lock().suspend();
}

unsafe fn force_unlock_process_manager() {
    if PROCESS_MANAGER.is_locked() { 
        PROCESS_MANAGER.force_unlock() 
    };
}
