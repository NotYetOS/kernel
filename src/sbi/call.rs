// SBI is a good thing
#![allow(unused)]

const SBI_SET_TIMER: u8 = 0x00;
const SBI_CONSOLE_PUTCHAR: u8 = 0x01;
const SBI_CONSOLE_GETCHAR: u8 = 0x02;
const SBI_CLEAR_IPI: u8 = 0x03;
const SBI_SEND_IPI: u8 = 0x04;
const SBI_REMOTE_FENCE_I: u8 = 0x05;
const SBI_REMOTE_SFENCE_VMA: u8 = 0x06;
const SBI_REMOTE_SFENCE_VMA_ASID: u8 = 0x07;
const SBI_SHUTDOWN: u8 = 0x08;

#[inline]
fn sbicall(id: u8, args: [usize; 3]) -> super::ret::Ret {
    let error: isize;
    let value: isize;

    // ecall to sbi
    unsafe {
        asm! {
            "ecall",
            lateout("x10") error, 
            lateout("x11") value,
            in("x10") args[0], in("x11") args[1], in("x12") args[2], 
            in("x17") id,
            options(nostack)
        }
    }

    super::ret::Ret {
        error: error.into(),
        value,
    }
}

pub fn console_putchar(ch: u8) {
    sbicall(SBI_CONSOLE_PUTCHAR, [ch as usize, 0, 0]);
}

pub fn console_getchar() -> isize {
    let ret = sbicall(SBI_CONSOLE_GETCHAR, [0, 0, 0]);
    ret.error.into()
}

pub fn shutdown() -> ! {
    sbicall(SBI_SHUTDOWN, [0, 0, 0]);
    unreachable!()
}