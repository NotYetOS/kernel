use super::pid::Pid;
use super::pid::alloc_pid;
use crate::mm::translated_refmut;
use crate::task::TaskUnit;
use crate::trap::get_trap_context;
use crate::trap::get_satp;
use alloc::vec::Vec;
use core::cell::RefCell;
use alloc::sync::{
    Arc, 
    Weak
};
use spin::{
    Mutex, 
    MutexGuard
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

    fn task_unit(self) -> TaskUnit {
        self.inner.into_inner().task_unit
    }

    fn drop_zombie_inner(&self) {
        self.inner_lock().drop_zombie()
    }

    fn inner_lock(&self) -> MutexGuard<ProcessUnitInner> {
        self.inner.lock()
    }

    pub fn pid(&self) -> usize {
        self.pid.value()
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

    pub fn replace(&self, src: ProcessUnit) {
        self.inner_lock().task_unit = src.task_unit();
    }

    pub fn waitpid(&self, pid: isize, exit_code: *mut i32) -> isize {
        let children = &mut self.inner_lock().children;
        let mut idx = 0;
        let pid = children.iter().enumerate().find(|&(_, child)| {
            child.pid() == pid as usize
        }).map_or(-1, |(i, child)| {
            if child.status() == TaskStatus::Zombie {
                idx = i;
                let satp = get_satp();
                let pa_exit = translated_refmut(
                    satp, 
                    exit_code
                );
                *pa_exit = child.exit_code();
                child.pid() as isize
            } else {
                -2
            }
        });
        if pid > 0 { drop(children.remove(idx)); }
        pid
    }

    pub fn wait(&self, exit_code: *mut i32) -> isize {
        let children = &mut self.inner_lock().children;
        let mut idx = 0;
        
        let pid = children.iter().enumerate().find(|&(_, child)| {
            child.status() == TaskStatus::Zombie
        }).map_or(-2, |(i, child)| {
            idx = i;
            let satp = get_satp();
            let pa_exit = translated_refmut(
                satp, 
                exit_code
            );
            *pa_exit = child.exit_code();
            child.pid() as isize
        });

        if pid > 0 { drop(children.remove(idx)); }

        pid
    }

    pub fn exit(&self) {
        let lock = self.inner_lock();
        match lock.parent.replace(None) {
            Some(weak) => {
                drop(lock);
                let parent = weak.upgrade().unwrap();
                parent.drop_zombie_inner();
            }
            None => {}
        }
    }
}

pub struct ProcessUnitInner {
    task_unit: TaskUnit,
    status: TaskStatus,
    exit_code: i32,
    parent: RefCell<Option<Weak<ProcessUnit>>>,
    children: Vec<Arc<ProcessUnit>>
}

impl ProcessUnitInner {
    pub fn new(task_unit: TaskUnit) -> Self {
        Self {
            task_unit,
            status: TaskStatus::Ready,
            exit_code: 0,
            parent: RefCell::new(None),
            children: Vec::new(),
        }
    }

    fn set_parent(
        &mut self, 
        parent: Weak<ProcessUnit>
    ) {
        self.parent.replace(Some(parent));
    }

    fn push_child(&mut self, 
        child: Arc<ProcessUnit>
    ) {
        self.children.push(child);
    }

    fn task_unit(&self) -> &TaskUnit {
        &self.task_unit
    }

    fn drop_zombie(&mut self) {
        let mut drop_children = Vec::new();
        self.children.iter().enumerate().for_each(|(
            index, 
            child
        )| {
            if TaskStatus::Zombie == child.status() {
                drop_children.push(index);
            }
        });
        drop_children.iter().for_each(|&i| {
            drop(self.children.remove(i))
        })
    } 
}

#[derive(Clone, Copy, PartialEq)]
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
            parent: RefCell::new(None),
            children: Vec::new(),
            ..*self
        }
    }
}

// impl Drop for ProcessUnit {
//     fn drop(&mut self) {
//         println!(
//             "Released a zombie process, pid={}, exit_code={}", 
//             self.pid.value(), 
//             self.exit_code()
//         );
//     }
// }
