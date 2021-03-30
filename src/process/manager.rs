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
    #[allow(unused)]
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
        loop { 
            if self.process.is_empty() { break; }
            self.run_inner(); 
        }
    }

    pub fn exit(&mut self) {
        // RAII, drop process if it is some 
        self.current = None;
        unsafe { _exit(); }
    }

    fn run_inner(&mut self) {
        match self.process.pop_front() {
            Some(mut process) => {
                let task_unit = process.task_unit();
                let satp = task_unit.satp;
                process.set_running();
                self.current = Some(process);
                unsafe { _load(satp); }
            }
            None => {}
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

unsafe fn force_unlock_process_manager() {
    if PROCESS_MANAGER.is_locked() { 
        PROCESS_MANAGER.force_unlock() 
    };
}
