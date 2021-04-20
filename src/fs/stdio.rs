use crate::alloc::string::ToString;
use crate::sbi::console_getchar;
use alloc::string::String;
use lazy_static::lazy_static;
use spin::Mutex;
use alloc::vec::Vec;
use super::{
    File, 
    UserBuffer
};

lazy_static! {
    static ref BYTE_BUFFER: Mutex<Vec<u8>> = Mutex::new(Vec::new());
}

lazy_static! {
    static ref CHARS: Mutex<String> = Mutex::new(String::new());
}

fn push_byte(byte: u8) {
    BYTE_BUFFER.lock().push(byte);
}

fn generate_chars() -> bool {
    let mut buf_lock = BYTE_BUFFER.lock();
    let mut chars_lock = CHARS.lock();
    let len = buf_lock.len();
    match core::str::from_utf8(&buf_lock[0..len]) {
        Ok(chars) => {
            chars_lock.push_str(chars);
            buf_lock.clear();
            true
        }
        Err(_) => {
            buf_lock.clear();
            false
        }
    }
}

fn chars_len() -> usize {
    CHARS.lock().len()
}

fn getchar_from_chars() -> char {
    let ch = CHARS.lock().remove(0);
    ch
}

pub struct Stdin;

impl File for Stdin {
    fn readable(&self) -> bool { true }
    fn writable(&self) -> bool { false }

    fn read(&self, mut buf: UserBuffer) -> usize {
        let mut ptr = buf.inner[0].as_mut_ptr();

        let mut read_func = || {
            let ch = getchar_from_chars();

            let chars = ch.to_string();
            for byte in chars.bytes() {
                unsafe {
                    ptr.write_volatile(byte);
                    ptr = ptr.add(1);
                }
            }
            
            ch.len_utf8()
        };

        if chars_len() != 0 {
            return read_func();
        }

        let first_byte = loop {
            let value = console_getchar();
            if value != -1 {
                break value as u8;
            }
        };

        push_byte(first_byte);

        loop {
            let value = console_getchar();
            if value != -1 {
                push_byte(value as u8);
            } else {
                if !generate_chars() {
                    let ret = -1;
                    return ret as usize;
                } else {
                    break;
                }
            }
        };

        read_func()
    }

    fn write(&self, _: UserBuffer) -> usize {
        panic!("can't write from stdout!");
    }
}

pub struct Stdout;

impl File for Stdout {
    fn readable(&self) -> bool { false }
    fn writable(&self) -> bool { true }

    fn read(&self, _: UserBuffer) -> usize {
        panic!("can't read from stdout!");
    }

    fn write(&self, buf: UserBuffer) -> usize {
        let vec = buf.concat();
        let str = String::from_utf8(vec).map_or(
            "".into(), 
            |str| str
        );
        print!("{}", str);
        buf.len()
    }
}
