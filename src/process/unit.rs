use core::ops::Deref;
use super::pid::Pid;
use super::pid::alloc_pid;
use crate::task::TaskUnit;
use crate::trap::get_trap_context;
use alloc::vec::Vec;
use spin::{Mutex, MutexGuard};
use alloc::sync::{
    Arc, 
    Weak
};

pub struct ProcessUnit {
    pid: Pid,
    inner: Mutex<ProcessUnitInner>
}

impl ProcessUnit {
    pub fn new(task_unit: TaskUnit) -> Self {
        Self {
            pid: alloc_pid(),
            inner: Mutex::new(
                ProcessUnitInner::new(task_unit)
            )
        }
    }

    pub fn fork(self: &Arc<ProcessUnit>) -> Arc<ProcessUnit> {
        let child = Arc::new(Self {
            pid: alloc_pid(),
            inner: Mutex::new(
                self.inner_lock().clone()
            ),
        });
        
        let cx = get_trap_context(child.satp());
        cx.x[10] = 0;
        cx.satp = child.satp();

        child.set_parent(
            Arc::downgrade(&Arc::clone(self))
        );

        self.push_child(Arc::clone(&child));
        child
    }

    fn set_parent(
        &self, 
        parent: Weak<ProcessUnit>
    ) {
        self.inner_lock().set_parent(parent);
    }

    fn push_child(&self, 
        child: Arc<ProcessUnit>
    ) {
        self.inner_lock().push_child(child);
    }

    pub fn pid(&self) -> usize {
        self.pid.value()
    }

    pub fn inner_lock(&self) -> MutexGuard<ProcessUnitInner> {
        self.inner.lock()
    }

    pub fn satp(&self) -> usize {
        self.inner_lock().task_unit().satp
    }

    pub fn set_zombie(&self, exit_code: i32) {
        self.inner_lock().exit_code = exit_code;
        self.inner_lock().status = TaskStatus::Zombie
    }

    pub fn set_suspend(&self) {
        self.inner_lock().status = TaskStatus::Suspend
    }

    pub fn set_running(&self) {
        self.inner_lock().status = TaskStatus::Running
    }

    pub fn status(&self) -> TaskStatus {
        self.inner_lock().status
    }

    pub fn exit_code(&self) -> i32 {
        self.inner_lock().exit_code
    }
}

pub struct ProcessUnitInner {
    task_unit: TaskUnit,
    status: TaskStatus,
    exit_code: i32,
    parent: Option<Weak<ProcessUnit>>,
    children: Vec<Arc<ProcessUnit>>
}

impl ProcessUnitInner {
    pub fn new(task_unit: TaskUnit) -> Self {
        Self {
            task_unit,
            status: TaskStatus::Ready,
            exit_code: 0,
            parent: None,
            children: Vec::new(),
        }
    }

    fn set_parent(
        &mut self, 
        parent: Weak<ProcessUnit>
    ) {
        self.parent = Some(parent);
    }

    fn push_child(&mut self, 
        child: Arc<ProcessUnit>
    ) {
        self.children.push(child);
    }

    pub fn task_unit(&self) -> &TaskUnit {
        &self.task_unit
    }
}

#[derive(Clone, Copy)]
pub enum TaskStatus {
    Ready,
    Running,
    Zombie,
    Suspend
}

impl Clone for ProcessUnitInner {
    fn clone(&self) -> Self {
        Self {
            task_unit: self.task_unit.clone(),
            parent: None,
            children: Vec::new(),
            ..*self
        }
    }
}

impl Drop for ProcessUnit {
    fn drop(&mut self) {
        println!(
            "Released a zombie process, pid={}, exit_code={}", 
            self.pid.value(), 
            self.exit_code()
        );
    }
}
