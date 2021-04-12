use crate::alloc::string::ToString;
use crate::sbi::console_getchar;
use super::{
    File, 
    UserBuffer
};

pub struct Stdin;

impl File for Stdin {
    fn read(&self, mut buf: UserBuffer) -> usize {
        let mut ptr = buf.inner[0].as_mut_ptr();
        let value = console_getchar();
        if value == -1 { return value as usize; }

        let ch = char::from_u32(value as u32).unwrap();
        if ch.len_utf8() > buf.len() { 
            let ret = -2; 
            return ret as usize; 
        }

        let chars = ch.to_string();
        for byte in chars.bytes() {
            unsafe {
                ptr.write_volatile(byte);
                ptr = ptr.add(1);
            }
        }
        
        ch.len_utf8()
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
