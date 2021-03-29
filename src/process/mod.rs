mod unit;
mod manager;

pub fn test() {
    use crate::task::TaskUnit;
    use unit::ProcessUnit;

    let path = "hello";
    let task = TaskUnit::new(path);
    let process = ProcessUnit::new(
        task
    );

    push_process(process);
    run();
}

pub use manager::*;
