use crate::task::TaskUnit;

pub struct ProcessUnit {
    task_unit: TaskUnit,
    status: TaskStatus
}

impl ProcessUnit {
    pub fn new(task_unit: TaskUnit) -> Self {
        Self {
            task_unit,
            status: TaskStatus::Ready
        }
    }

    pub fn task_unit(&self) -> &TaskUnit {
        &self.task_unit
    }

    pub fn set_zombie(&mut self) {
        self.status = TaskStatus::Zombie
    }

    pub fn set_suspend(&mut self) {
        self.status = TaskStatus::Suspend
    }

    pub fn set_running(&mut self) {
        self.status = TaskStatus::Running
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
