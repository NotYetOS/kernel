use super::unit::ProcessUnit;
use core::cell::RefCell;
use alloc::collections::VecDeque;
use lazy_static::lazy_static;
use spin::Mutex;
use alloc::sync::Arc;
use alloc::vec::Vec;
use alloc::string::String;
use crate::fs::ROOT;
use crate::task::TaskUnit;

global_asm!(include_str!("process.s"));

extern "C" {
    fn _load(satp: usize); 
    fn _ret();
    fn _save_call_context(satp: usize);
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
            current: RefCell::new(None),
        }
    }

    pub fn push_process(
        &mut self, 
        process: Arc<ProcessUnit>
    ) {
        self.process.push_back(process);
    }

    pub fn pop_process(
        &mut self,
    ) -> Option<Arc<ProcessUnit>> {
        self.process.pop_front()
    }

    pub fn exit_current(&mut self, exit_code: i32) {
        let out = self.pop_process().map_or_else(
            || self.take_current().unwrap(), 
            |process| self.replace(process).unwrap()
        );
        out.set_zombie(exit_code);
    }

    pub fn suspend_current(&mut self) {
        self.pop_process().map(|process| {
            let last = self.replace(
                process
            ).unwrap();
            last.set_suspend();
            self.push_process(last);
        });
    }

    pub fn fork_current(&mut self) -> usize {
        let current = self.current().unwrap();
        let child = current.fork();
        let pid = child.pid();
        self.push_process(child);
        pid
    }

    pub fn waitpid_current(&self, 
        pid: isize,
        exit_code: *mut i32
    ) -> isize {
        let current = self.current().unwrap();

        match pid {
            -1 => current.wait(exit_code),
            other @ _ => current.waitpid(other, exit_code)
        }
    }

    pub fn save_call_context(&self) {
        let current = self.current().unwrap();
        unsafe { _save_call_context(current.satp()) }
    }

    pub fn exec(&self, path: &str, other_args: Vec<String>) -> isize {
        let mut elf_data = Vec::new();
        let root_gurad = ROOT.lock();
        let bin_dir = root_gurad.cd("bin").unwrap();

        let gen_process = |elf_data: &[u8]| {
            let task = TaskUnit::new(&elf_data);
            let new = ProcessUnit::new(task);
            new.push_args(&path, other_args);
            match self.current() {
                Some(last) => {
                    last.replace(new);
                    self.replace(last);
                }
                None => {
                    self.replace(
                        Arc::new(new)
                    );
                }
            }
        };

        match bin_dir.open_file(&path) {
            Ok(bin) => {
                let len = bin.read_to_vec(
                    &mut elf_data
                ).unwrap();
                gen_process(&elf_data[0..len]);
                drop(root_gurad);
                0
            }
            Err(_) => -1
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

    pub fn run_inner(&mut self) -> bool {
        match self.current.get_mut() {
            Some(process) => {
                process.set_running();
                load(process.satp());
                true
            }
            None => false
        }
    }

    pub fn run(&mut self) {
        loop {
            if !self.run_inner() {
                break;
            }
        }
    }
    
    pub fn current(
        &self
    ) -> Option<Arc<ProcessUnit>> {
        self.current.replace(None).map_or(
            None, 
            |current| {
                let current_clone = current.clone();
                self.replace(current);
                Some(current_clone)
            }
        )
    }

    pub fn take_current(
        &self
    ) -> Option<Arc<ProcessUnit>> {
        self.current.replace(None)
    }

    pub fn replace(
        &self, 
        src: Arc<ProcessUnit>
    ) -> Option<Arc<ProcessUnit>> {
        self.current.replace(Some(src))
    }
}

lazy_static! {
    pub static ref PROCESS_MANAGER: Mutex<ProcessManager> = {
        Mutex::new(ProcessManager::new())
    };
}

pub fn save_call_context() {
    PROCESS_MANAGER.lock().save_call_context();
}

pub fn run() {
    PROCESS_MANAGER.lock().run()
}

unsafe fn force_unlock_process_manager() {
    if PROCESS_MANAGER.is_locked() { 
        PROCESS_MANAGER.force_unlock() 
    };
}
