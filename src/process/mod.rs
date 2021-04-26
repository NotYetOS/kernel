mod unit;
mod manager;
mod pid;

use crate::syscall;
use alloc::vec;
use alloc::vec::Vec;

pub fn start() {
    println!("");
    println!("[test] process");
    println!("----------------------->");
    
    exec("shell", vec![]);
    run();

    println!("<-----------------------");
    println!("[passed] process test");
}

pub fn exec(path: &'static str, mut args: Vec<&str>) {
    use syscall::sys_exec;
    args.insert(0, path);
    let new_args = args.join(" ");
    sys_exec(new_args.as_ptr(), new_args.len());
}

pub use manager::*;
pub use unit::*;
pub use pid::*;
pub use syscall::sys_yield as suspend;
pub use syscall::sys_exit as exit;
