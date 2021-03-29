use core::i32;
use crate::process::*;

pub fn sys_exit(exit_code: i32) -> ! {
    exit();
    panic!();
}
