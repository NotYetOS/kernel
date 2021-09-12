use super::pid::alloc_pid;
use super::pid::Pid;
use crate::context::get_context;
use crate::fs::File;
use crate::mm::MemorySet;
use crate::trap::get_satp;

use crate::fs::{Stdin, Stdout};
use crate::mm::translated_refmut;
use alloc::string::String;
use alloc::sync::{Arc, Weak};
use alloc::vec;
use alloc::vec::Vec;
use riscv::register::sstatus::SPP;
use spin::{Mutex, MutexGuard};

#[derive(Clone, Copy, PartialEq)]
pub enum ProcessStatus {
    Ready,
    Running,
    Zombie,
    Suspend,
}

pub struct ProcessUnit {
    pid: Pid,
    inner: Mutex<ProcessUnitInner>,
}

impl ProcessUnit {
    pub fn new(elf_data: &[u8]) -> Self {
        Self {
            pid: alloc_pid(),
            inner: Mutex::new(ProcessUnitInner::new(elf_data)),
        }
    }

    pub fn fork_self(self: &Arc<ProcessUnit>) -> Arc<ProcessUnit> {
        let child = Arc::new(Self {
            pid: alloc_pid(),
            inner: Mutex::new(self.inner_lock().clone()),
        });

        let cx = get_context(child.satp());
        cx.a0 = 0;
        cx.satp = child.satp();

        child.set_parent(Arc::downgrade(self));

        self.push_child(Arc::clone(&child));
        child
    }

    pub fn set_parent(&self, parent: Weak<ProcessUnit>) {
        self.inner_lock().set_parent(parent);
    }

    pub fn push_child(&self, child: Arc<ProcessUnit>) {
        self.inner_lock().push_child(child);
    }

    pub fn alloc_fd(&self, fd_table: &mut Vec<Option<Arc<dyn File>>>) -> usize {
        let fd_end = fd_table.len();

        (0..fd_end)
            .find(|&fd| fd_table.get(fd).unwrap().is_none())
            .map_or_else(
                || {
                    fd_table.push(None);
                    fd_end
                },
                |fd| fd,
            )
    }

    pub fn inner_lock(&self) -> MutexGuard<ProcessUnitInner> {
        self.inner.lock()
    }

    pub fn pid(&self) -> usize {
        self.pid.value()
    }

    pub fn satp(&self) -> usize {
        self.inner_lock().satp
    }

    pub fn set_zombie(&self, exit_code: i32) {
        self.inner_lock().exit_code = exit_code;
        self.inner_lock().status = ProcessStatus::Zombie;
    }

    pub fn set_suspend(&self) {
        self.inner_lock().status = ProcessStatus::Suspend;
    }

    pub fn set_running(&self) {
        self.inner_lock().status = ProcessStatus::Running;
    }

    pub fn status(&self) -> ProcessStatus {
        self.inner_lock().status
    }

    pub fn exit_code(&self) -> i32 {
        self.inner_lock().exit_code
    }

    pub fn push_args(&self, path: &str, mut args: Vec<String>) {
        let satp = self.satp();
        let mut len = 0;
        let cx = get_context(satp);
        args.insert(0, path.into());
        cx.a0 = args.len();
        args.iter().for_each(|arg| len += arg.len());
        let mut sp = cx.sp;
        sp -= len + args.len();
        let mut addr = sp;

        args.iter().for_each(|arg| {
            for byte in arg.bytes() {
                *translated_refmut(satp, addr as *mut u8) = byte;
                addr += 1;
            }
            *translated_refmut(satp, addr as *mut u8) = '\0' as u8;
            addr += 1;
        });

        cx.sp = sp;
        cx.a0 = args.len();
        cx.a1 = sp;
    }

    pub fn waitpid(&self, pid: isize, exit_code: *mut i32) -> isize {
        let children = &mut self.inner_lock().children;

        (0..children.len())
            .find(|&idx| children.get(idx).unwrap().pid() == pid as usize)
            .map_or(-1, |idx| {
                let child = children.remove(idx);
                match child.status() {
                    ProcessStatus::Zombie => {
                        let satp = get_satp();
                        let pa_exit = translated_refmut(satp, exit_code);
                        *pa_exit = child.exit_code();
                        child.pid() as isize
                    }
                    _ => {
                        children.push(child);
                        -2
                    }
                }
            })
    }

    pub fn wait(&self, exit_code: *mut i32) -> isize {
        let children = &mut self.inner_lock().children;
        if children.is_empty() {
            return -1;
        }

        (0..children.len())
            .find(|&idx| children.get(idx).unwrap().status() == ProcessStatus::Zombie)
            .map_or(-2, |idx| {
                let child = children.remove(idx);
                let satp = get_satp();
                let pa_exit = translated_refmut(satp, exit_code);
                *pa_exit = child.exit_code();
                child.pid() as isize
            })
    }
}

pub struct ProcessUnitInner {
    satp: usize,
    mem_set: MemorySet,
    status: ProcessStatus,
    exit_code: i32,
    #[allow(unused)]
    parent: Option<Weak<ProcessUnit>>,
    children: Vec<Arc<ProcessUnit>>,
    fd_table: Vec<Option<Arc<dyn File>>>,
}

impl ProcessUnitInner {
    pub fn new(elf_data: &[u8]) -> Self {
        let mem_set = MemorySet::from_elf(SPP::User, &elf_data);

        Self {
            satp: mem_set.satp_bits(),
            mem_set,
            status: ProcessStatus::Ready,
            exit_code: 0,
            parent: None,
            children: Vec::new(),
            fd_table: vec![
                Some(Arc::new(Stdin)),
                Some(Arc::new(Stdout)),
                Some(Arc::new(Stdout)),
            ],
        }
    }

    pub fn set_parent(&mut self, parent: Weak<ProcessUnit>) {
        self.parent = Some(parent);
    }

    pub fn push_child(&mut self, child: Arc<ProcessUnit>) {
        self.children.push(child);
    }

    pub fn fd_table(&self) -> &Vec<Option<Arc<dyn File>>> {
        &self.fd_table
    }

    pub fn fd_table_mut(&mut self) -> &mut Vec<Option<Arc<dyn File>>> {
        &mut self.fd_table
    }
}

impl Clone for ProcessUnitInner {
    fn clone(&self) -> Self {
        let mem_set = self.mem_set.clone();

        Self {
            satp: mem_set.satp_bits(),
            mem_set,
            parent: None,
            children: Vec::new(),
            fd_table: self.fd_table.clone(),
            ..*self
        }
    }
}
