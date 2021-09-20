use crate::context::get_context;
use crate::fs::ROOT;
use crate::mm::{translated_ref, translated_str};
use crate::process::alloc_pid;
use crate::process::current_process;
use crate::process::load;
use crate::process::pop_process;
use crate::process::push_process;
use crate::process::set_current_process;
use crate::process::take_current_process;
use crate::process::{self, ProcessUnit};
use crate::trap::get_satp;
use crate::trap::get_time_ms;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use process::PROCESS_MANAGER;
use spin::Mutex;

pub fn sys_exit(exit_code: i32) -> isize {
    take_current_process().map_or(-1, |process| {
        process.set_zombie(exit_code);
        0
    });

    pop_process().map_or(-1, |process| {
            let satp = process.satp();
            set_current_process(process);
            load(satp);
            0
        },
    )
}

pub fn sys_yield() -> isize {
    pop_process().map_or(-1, |process| {
        if let Some(current) = current_process() {
            current.set_suspend();
            push_process(current);
        }
        let satp = process.satp();
        set_current_process(process);
        load(satp);
        0
    })
}

pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}

pub fn sys_getpid() -> isize {
    match current_process() {
        Some(process) => process.pid() as isize,
        None => -1,
    }
}

pub fn sys_fork() -> isize {
    match current_process() {
        Some(process) => {
            let child = process.fork_self();
            let pid = child.pid();
            push_process(child);
            pid as isize
        }
        None => -1,
    }
}

pub fn sys_exec(args: *const u8, len: usize) -> isize {
    let satp = get_satp();
    let mut args = translated_str(satp, args, len);
    args.push_str(" ");
    let mut other_args: Vec<String> = args
        .split(" ")
        .filter(|arg| !arg.is_empty())
        .map(|arg| arg.into())
        .collect();
    let path = other_args.remove(0);

    let mut elf_data = Vec::new();
    let root_gurad = ROOT.lock();
    let bin_dir = root_gurad.cd("bin").unwrap();

    let gen_process = |elf_data: &[u8]| {
        let new_process = ProcessUnit::new(elf_data);
        new_process.push_args(&path, other_args);
        let satp = new_process.satp();
        match take_current_process() {
            Some(process) => {
                process.set_suspend();
                push_process(process);
                set_current_process(Arc::new(new_process));
            }
            None => set_current_process(Arc::new(new_process)),
        }
        satp
    };

    match bin_dir.open_file(&path) {
        Ok(bin) => {
            let len = bin.read_to_vec(&mut elf_data).unwrap();
            drop(root_gurad);
            let satp = gen_process(&elf_data[0..len]);
            load(satp);
            0
        }
        Err(_) => -1,
    }
}

pub fn sys_waitpid(pid: isize, exit_code: *mut i32) -> isize {
    match current_process() {
        Some(process) => match pid {
            -1 => process.wait(exit_code),
            other => process.waitpid(other, exit_code),
        },
        None => -1,
    }
}
