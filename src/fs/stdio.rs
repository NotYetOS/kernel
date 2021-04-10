use crate::sbi::console_getchar;
use super::{
    File, 
    UserBuffer
};

pub struct Stdin;

impl File for Stdin {
    fn read(&self, mut buf: UserBuffer) -> usize {
        let ch_value = console_getchar() as u32;
        let bytes = u32::to_le_bytes(ch_value);
        let mut ptr = buf.inner[0].as_mut_ptr();
        (0..buf.len()).for_each(|idx| {
            unsafe {
                ptr.write_volatile(bytes[idx]);
                ptr = ptr.add(1);
            }
        });
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
