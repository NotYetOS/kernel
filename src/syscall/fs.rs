use crate::mm::*;
use crate::trap::*;

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    let satp = get_satp();
    translated_get_char(satp, buf, len);
    0
}

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let satp = get_satp();
    let string = translated_str(satp, buf, len);
    print!("{}", string);
    0
}
