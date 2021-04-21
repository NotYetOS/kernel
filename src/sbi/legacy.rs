#![allow(unused)]

use super::call::sbicall;

const SBI_SET_TIMER: usize = 0x00;
const SBI_CONSOLE_PUTCHAR: usize = 0x01;
const SBI_CONSOLE_GETCHAR: usize = 0x02;
const SBI_CLEAR_IPI: usize = 0x03;
const SBI_SEND_IPI: usize = 0x04;
const SBI_REMOTE_FENCE_I: usize = 0x05;
const SBI_REMOTE_SFENCE_VMA: usize = 0x06;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 0x07;
const SBI_SHUTDOWN: usize = 0x08;

pub fn set_timer(timer: usize) {
    sbicall(SBI_SET_TIMER, 0, [timer, 0, 0, 0, 0]);
}

pub fn console_putchar(ch: u8) {
    sbicall(SBI_CONSOLE_PUTCHAR, 0, [ch as usize, 0, 0, 0, 0]);
}

pub fn console_getchar() -> isize {
    let ret = sbicall(SBI_CONSOLE_GETCHAR, 0, [0, 0, 0, 0, 0]);
    ret.error.into()
}
