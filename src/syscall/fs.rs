use crate::mm::*;
use crate::trap::*;
use crate::process;
use crate::fs::OpenFlags;
use crate::process::PROCESS_MANAGER;
use crate::fs::{
    UserBuffer, 
    make_pipe, 
    open_file
};

pub fn sys_dup(fd: usize) -> isize {
    todo!()
}

pub fn sys_open(path: *const u8, len: usize, flags: u32) -> isize {
    let lock = PROCESS_MANAGER.lock();
    let current = lock.current().unwrap();

    let satp = get_satp();
    let path = translated_str(satp, path, len);
    let mut inner = current.inner_lock();
    let fd_table = inner.fd_table_mut();
    let flags = OpenFlags::from_bits(flags).unwrap();
    open_file(&path, flags).map_or(-1, |file| {
        let fd = current.alloc_fd(fd_table);
        *fd_table.get_mut(fd).unwrap() = Some(file);
        fd as isize
    })
}

pub fn sys_close(fd: usize) -> isize {
    let lock = PROCESS_MANAGER.lock();
    let current = lock.current().unwrap();

    let mut inner = current.inner_lock();
    match inner.fd_table_mut().get_mut(fd) {
        Some(io) => {
            io.take();
            0
        }
        None => -1
    }
}

pub fn sys_pipe(ptr: *mut usize) -> isize {
    let lock = PROCESS_MANAGER.lock();
    let current = lock.current().unwrap();

    let satp = current.satp();
    let mut inner = current.inner_lock();
    let fd_table = inner.fd_table_mut();
    let (read, write) = make_pipe();

    let read_fd = current.alloc_fd(fd_table);
    *fd_table.get_mut(read_fd).unwrap() = Some(read);
    let write_fd = current.alloc_fd(fd_table);
    *fd_table.get_mut(write_fd).unwrap() = Some(write);;

    *translated_refmut(satp, ptr) = read_fd;
    *translated_refmut(
        satp, 
        unsafe { ptr.add(1) }
    ) = write_fd;

    0
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    let lock = PROCESS_MANAGER.lock();
    let current = lock.current().unwrap();

    let satp = current.satp();
    let inner = current.inner_lock();
    inner.fd_table().get(fd).map_or(-1, |option| {
        option.as_ref().map_or(-1, |io| {
            let buf = UserBuffer::new(
                translated_byte_buffer(satp, buf, len)
            );
            io.read(buf) as isize
        })
    })
}

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let lock = PROCESS_MANAGER.lock();
    let current = lock.current().unwrap();

    let satp = current.satp();
    let inner = current.inner_lock();
    inner.fd_table().get(fd).map_or(-1, |option| {
        option.as_ref().map_or(-1, |io| {
            let buf = UserBuffer::new(
                translated_byte_buffer(satp, buf, len)
            );
            io.write(buf) as isize
        })
    })
}
