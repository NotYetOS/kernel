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
}

pub enum TaskStatus {
    Ready,
    Running,
    Zombie,
}