use super::unit::ProcessUnit;
use super::unit::TaskStatus;
use core::cell::RefCell;
use alloc::collections::VecDeque;
use lazy_static::lazy_static;
use spin::Mutex;
use crate::task::TaskUnit;
use alloc::vec::Vec;
use crate::fs::{
    ROOT,
    File,
};
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

    pub fn exit(&mut self, code: i32) {
        let out = match self.pop_process() {
            Some(process) => {
                self.replace(process).unwrap()
            },
            None => {
                self.current().unwrap()
            }
        };
        out.set_zombie(code);
        // out.exit();
        drop(out);
    }

    pub fn suspend(&mut self) {
        match self.process.pop_front() {
            Some(process) => {
                let last = self.replace(
                    process
                ).unwrap();
                last.set_suspend();
                self.push_process(last);
            }
            None => {}
        }
    }

    pub fn save_call_context(&self) {
        let current = self.current().unwrap();
        unsafe { _save_call_context(current.satp()) }
        self.replace(current);
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
        let process = self.current().unwrap();
        let child = process.fork();
        let pid = child.pid();
        self.push_process(child);
        self.replace(process);
        pid
    }

    pub fn exec(&mut self, path: &str) -> isize {
        let mut elf_data = Vec::new();
        let root_gurad = ROOT.lock();
        let bin_dir = root_gurad.cd("bin").unwrap();

        let gen_process= |elf_data: &[u8]| {
            let task = TaskUnit::new(&elf_data);
            let new = ProcessUnit::new(task);
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

        match bin_dir.open_file(path) {
            Ok(bin) => {
                let len = bin.read_to_vec(&mut elf_data).unwrap();
                gen_process(&elf_data[0..len]);
                drop(root_gurad);
                0
            }
            Err(_) => -1
        }
    }

    pub fn waitpid(&self, pid: isize, exit_code: *mut i32) -> isize {
        let current = self.current().unwrap();
        let pid = match pid {
            -1 => current.wait(exit_code),
            other @ _ => current.waitpid(other, exit_code)
        };
        self.replace(current);
        pid
    }

    pub fn read(&self, fd: usize, buf: *const u8, len: usize) -> isize {
        let current = self.current().unwrap();
        let ret = current.read(fd, buf, len);
        self.replace(current);
        ret
    }

    pub fn write(&self, fd: usize, buf: *const u8, len: usize) -> isize {
        let current = self.current().unwrap();
        let ret = current.write(fd, buf, len);
        self.replace(current);
        ret
    }

    pub fn close(&self, fd: usize) -> isize {
        let current = self.current().unwrap();
        let ret = current.close(fd);
        self.replace(current);
        ret
    }

    pub fn pipe(&self, pipe: *mut usize) -> isize {
        let current = self.current().unwrap();
        let ret = current.pipe(pipe);
        self.replace(current);
        ret
    }

    pub fn run_inner(&mut self) -> bool {
        match self.current.get_mut() {
            Some(process) => {
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
    
    fn current(
        &self
    ) -> Option<Arc<ProcessUnit>> {
        self.current.replace(None)
    }

    fn replace(
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

pub fn push_process(process: Arc<ProcessUnit>) {
    PROCESS_MANAGER.lock().push_process(process);
}

pub fn exit(code: i32) {
    PROCESS_MANAGER.lock().exit(code);
}

pub fn suspend() {
    PROCESS_MANAGER.lock().suspend();
}

pub fn save_call_context() {
    PROCESS_MANAGER.lock().save_call_context();
}

pub fn getpid() -> usize {
    PROCESS_MANAGER.lock().pid()
}

pub fn fork() -> usize {
    PROCESS_MANAGER.lock().fork()
}

pub fn exec(path: &str) -> isize {
    PROCESS_MANAGER.lock().exec(path)
}

pub fn waitpid(pid: isize, exit_code: *mut i32) -> isize {
    PROCESS_MANAGER.lock().waitpid(pid, exit_code)
}

pub fn read(fd: usize, buf: *const u8, len: usize) -> isize {
    PROCESS_MANAGER.lock().read(fd, buf, len)
}

pub fn write(fd: usize, buf: *const u8, len: usize) -> isize {
    PROCESS_MANAGER.lock().write(fd, buf, len)
}

pub fn close(fd: usize) -> isize {
    PROCESS_MANAGER.lock().close(fd)
}

pub fn pipe(pipe: *mut usize) -> isize {
    PROCESS_MANAGER.lock().pipe(pipe)
}

pub fn run() {
    PROCESS_MANAGER.lock().run()
}

unsafe fn force_unlock_process_manager() {
    if PROCESS_MANAGER.is_locked() { 
        PROCESS_MANAGER.force_unlock() 
    };
}
