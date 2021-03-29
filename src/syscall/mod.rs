#![allow(unused)]

const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

mod fs;

pub fn syscall(id: usize, args: [usize; 3]) -> isize {
    match id {
        // SYSCALL_READ => sys_read(args[0], args[1] as *const u8, args[2]),
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        // SYSCALL_EXIT => sys_exit(args[0] as i32),
        _ => panic!("unsupported syscall_id: {}", id),
    }
}

use fs::*;
