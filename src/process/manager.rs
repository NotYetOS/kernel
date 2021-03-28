use alloc::collections::VecDeque;
use super::unit::ProcessUnit;
use lazy_static::lazy_static;
use spin::Mutex;

global_asm!(include_str!("load.s"));

extern "C" {
    fn _load(satp: usize); 
}

pub struct ProcessManager {
    process: VecDeque<ProcessUnit>
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            process: VecDeque::new()
        }
    }

    pub fn push_process(&mut self, process: ProcessUnit) {
        self.process.push_front(process);
    }

    pub fn run(&mut self) {
        match self.process.pop_front() {
            Some(process) => {
                let task_unit = process.task_unit();
                unsafe {
                    _load(task_unit.satp);
                }
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
