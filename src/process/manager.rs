use core::borrow::BorrowMut;
use alloc::collections::VecDeque;
use super::unit::ProcessUnit;
use lazy_static::lazy_static;
use spin::Mutex;

global_asm!(include_str!("process.s"));

extern "C" {
    fn _load(satp: usize); 
    fn _exit();
}

pub struct ProcessManager {
    process: VecDeque<ProcessUnit>,
    current: Option<ProcessUnit>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            process: VecDeque::new(),
            current: None
        }
    }

    pub fn push_process(&mut self, process: ProcessUnit) {
        self.process.push_front(process);
    }

    pub fn run(&mut self) {
        match self.process.pop_front() {
            Some(process) => {
                let task_unit = process.task_unit();
                let satp = task_unit.satp;
                self.current = Some(process);
                unsafe { _load(satp); }
            }
            None => panic!("no task can be run")
        }
    }

    pub fn exit(&mut self) {
        match self.current.borrow_mut() {
            Some(process) => {
                process.set_zombie();
                unsafe { _exit() }
            }
            None => panic!("no task can be exited")
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
    unsafe {
        PROCESS_MANAGER.force_unlock();
    }
    PROCESS_MANAGER.lock().exit()
}
