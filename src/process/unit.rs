use crate::task::TaskUnit;
use super::pid::Pid;
use super::pid::alloc_pid;

pub struct ProcessUnit {
    pid: Pid,
    task_unit: TaskUnit,
    status: TaskStatus,
    exit_code: i32
}

impl ProcessUnit {
    pub fn new(task_unit: TaskUnit) -> Self {
        Self {
            pid: alloc_pid(),
            task_unit,
            status: TaskStatus::Ready,
            exit_code: 0,
        }
    }

    pub fn task_unit(&self) -> &TaskUnit {
        &self.task_unit
    }

    pub fn set_zombie(&mut self, exit_code: i32) {
        self.exit_code = exit_code;
        self.status = TaskStatus::Zombie
    }

    pub fn set_suspend(&mut self) {
        self.status = TaskStatus::Suspend
    }

    pub fn set_running(&mut self) {
        self.status = TaskStatus::Running
    }

    pub fn pid(&self) -> usize {
        self.pid.value()
    }

    pub fn status(&self) -> TaskStatus {
        self.status
    }
}

#[derive(Clone, Copy)]
pub enum TaskStatus {
    Ready,
    Running,
    Zombie,
    Suspend
}

impl Drop for ProcessUnit {
    fn drop(&mut self) {
        println!(
            "Released a zombie process, pid={}, exit_code={}", 
            self.pid.value(), 
            self.exit_code
        );
    }
}
