use crate::mm::*;
use crate::trap::*;
use crate::process;

pub fn sys_close(fd: usize) -> isize {
    process::close(fd)
}

pub fn sys_pipe(pipe: *mut usize) -> isize {
    process::pipe(pipe)
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    process::read(fd, buf, len)
}

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    process::write(fd, buf, len)
}
