use alloc::sync::Arc;
use alloc::vec::Vec;
use alloc::string::String;
use process::PROCESS_MANAGER;
use spin::Mutex;
use crate::context::get_context;
use crate::task::TaskUnit;
use crate::trap::get_satp;
use crate::trap::get_time_ms;
use crate::fs::ROOT;
use crate::process::alloc_pid;
use crate::process::{
    self, 
    ProcessUnit
};
use crate::mm::{
    translated_str,
    translated_ref
};

pub fn sys_exit(exit_code: i32) -> isize {
    PROCESS_MANAGER.lock().exit_current(exit_code);
    0
}

pub fn sys_yield() -> isize {
    PROCESS_MANAGER.lock().suspend_current();
    0
}

pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}

pub fn sys_getpid() -> isize {
    PROCESS_MANAGER.lock().pid() as isize
}

pub fn sys_fork() -> isize {
    PROCESS_MANAGER.lock().fork_current() as isize
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
    PROCESS_MANAGER.lock().exec(&path, other_args)
}

pub fn sys_waitpid(pid: isize, exit_code: *mut i32) -> isize {
    PROCESS_MANAGER.lock().waitpid_current(pid, exit_code) as isize
}
