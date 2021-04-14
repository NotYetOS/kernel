use crate::process;
use crate::trap::get_satp;
use crate::trap::get_time_ms;
use crate::mm::{
    translated_str,
    translated_ref
};
use alloc::vec::Vec;
use alloc::string::String;

pub fn sys_exit(exit_code: i32) -> isize {
    process::exit(exit_code);
    0
}

pub fn sys_yield() -> isize {
    process::suspend();
    0
}

pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}

pub fn sys_getpid() -> isize {
    process::getpid() as isize
}

pub fn sys_fork() -> isize {
    process::fork() as isize
}

pub fn sys_exec(args: *const u8, len: usize) -> isize {
    let satp = get_satp();
    let mut args = translated_str(satp, args, len);
    args.push_str(" ");
    let mut other_args: Vec<String> = args.split(" ")
                                .filter(|arg| !arg.is_empty())
                                .map(|arg| arg.into())
                                .collect();
    let path = other_args.remove(0);
    process::exec(&path, other_args) as isize
}

pub fn sys_waitpid(pid: isize, exit_code: *mut i32) -> isize {
    process::waitpid(pid, exit_code)
}
