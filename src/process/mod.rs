mod unit;
mod manager;
mod pid;

use crate::syscall;
use alloc::vec;
use alloc::vec::Vec;
use alloc::string::String;

pub fn start() {
    println!("");
    println!("[test] process");
    println!("----------------------->");
    
    exec("shell", vec![]);
    run();

    println!("<-----------------------");
    println!("[passed] process test");
}

pub fn exec(path: &str, args: Vec<&str>) {
    use syscall::sys_exec;

    let mut new_args: String = path.into();
    new_args.push(' ');
    args.iter().for_each(|arg| {
        new_args.push_str(arg);
        new_args.push(' ');
    });
    sys_exec(new_args.as_ptr(), new_args.len());
}

pub use manager::*;
pub use unit::*;
pub use pid::*;
pub use syscall::sys_yield as suspend;
pub use syscall::sys_exit as exit;
