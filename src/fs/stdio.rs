use crate::sbi::console_getchar;
use super::{
    File, 
    UserBuffer
};

pub struct Stdin;

impl File for Stdin {
    fn read(&self, mut buf: UserBuffer) -> usize {
        let ch = console_getchar();
        let ptr = buf.inner[0].as_mut_ptr();
        unsafe {
            ptr.write_volatile(ch as u8);
        }
        1
    }

    fn write(&self, _: UserBuffer) -> usize {
        panic!("can't write from stdout!");
    }
}

pub struct Stdout;

impl File for Stdout {
    fn read(&self, _: UserBuffer) -> usize {
        panic!("can't read from stdout!");
    }

    fn write(&self, buf: UserBuffer) -> usize {
        for buf in buf.inner.iter() {
            let str = core::str::from_utf8(buf).unwrap();
            print!("{}", str);
        }
        buf.len()
    }
}
