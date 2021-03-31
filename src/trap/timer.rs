use crate::config::*;
use crate::sbi::*;
use riscv::register::{
    sie, 
    time
};

const TICKS_PER_SEC: usize = 100;
const MSEC_PER_SEC: usize = 1000;

pub fn enable() {
    unsafe { sie::set_stimer(); }
}

pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}

fn get_time() -> usize {
    time::read()
}

fn get_time_ms() -> usize {
    get_time() / (CLOCK_FREQ / MSEC_PER_SEC)
}
