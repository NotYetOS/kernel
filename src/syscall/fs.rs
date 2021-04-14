use crate::mm::*;
use crate::trap::*;
use crate::process;
use crate::fs::OpenFlags;

pub fn sys_dup(fd: usize) -> isize {
    todo!()
}

pub fn sys_open(path: *const u8, len: usize, flags: u32) -> isize {
    let satp = get_satp();
    let path = translated_str(satp, path, len);
    process::open(&path, OpenFlags::from_bits(flags).unwrap()) as isize
}

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
