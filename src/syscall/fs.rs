use crate::mm::*;
use crate::trap::*;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let satp = get_satp();
    let string = translated_str(satp, buf, len);
    print!("{}", string);
    0
}