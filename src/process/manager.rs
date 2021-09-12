use super::unit::ProcessUnit;
use crate::trap::get_kernel_satp;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use core::cell::RefCell;
use lazy_static::lazy_static;
use spin::Mutex;

global_asm!(include_str!("process.s"));

extern "C" {
    fn _load(user_satp: usize);
    fn _ret();
    fn _save_call_context(user_satp: usize, kernel_satp: usize);
}

pub fn load(satp: usize) {
    unsafe {
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
            current: RefCell::new(None),
        }
    }

    pub fn push_process(&mut self, process: Arc<ProcessUnit>) {
        self.process.push_back(process);
    }

    pub fn pop_process(&mut self) -> Option<Arc<ProcessUnit>> {
        self.process.pop_front()
    }

    pub fn save_call_context(&self) {
        let current = self.current().unwrap();
        unsafe { _save_call_context(current.satp(), get_kernel_satp()) }
    }

    pub fn run_inner(&mut self) -> bool {
        match self.current.get_mut() {
            Some(process) => {
                process.set_running();
                load(process.satp());
                true
            }
            None => false,
        }
    }

    pub fn run(&mut self) {
        loop {
            if !self.run_inner() {
                break;
            }
        }
    }

    pub fn current(&self) -> Option<Arc<ProcessUnit>> {
        self.current.borrow().as_ref().map(|unit| Arc::clone(&unit))
    }

    pub fn take_current(&self) -> Option<Arc<ProcessUnit>> {
        self.current.take()
    }

    pub fn set_current(&self, process: Arc<ProcessUnit>) {
        *self.current.borrow_mut() = Some(process)
    }
}

lazy_static! {
    pub static ref PROCESS_MANAGER: Mutex<ProcessManager> = Mutex::new(ProcessManager::new());
}

pub fn take_current_process() -> Option<Arc<ProcessUnit>> {
    PROCESS_MANAGER.lock().take_current()
}

pub fn current_process() -> Option<Arc<ProcessUnit>> {
    PROCESS_MANAGER.lock().current()
}

pub fn set_current_process(process: Arc<ProcessUnit>) {
    PROCESS_MANAGER.lock().set_current(process)
}

pub fn push_process(process: Arc<ProcessUnit>) {
    PROCESS_MANAGER.lock().push_process(process)
}

pub fn pop_process() -> Option<Arc<ProcessUnit>> {
    PROCESS_MANAGER.lock().pop_process()
}

pub fn save_call_context() {
    PROCESS_MANAGER.lock().save_call_context();
}

pub fn run() {
    PROCESS_MANAGER.lock().run()
}
