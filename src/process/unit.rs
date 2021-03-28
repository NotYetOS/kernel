use crate::task::TaskUnit;

pub struct ProcessUnit {
    task_unit: TaskUnit,
}

impl ProcessUnit {
    pub fn new(task_unit: TaskUnit) -> Self {
        Self {
            task_unit
        }
    }

    pub fn task_unit(&self) -> &TaskUnit {
        &self.task_unit
    }
}
