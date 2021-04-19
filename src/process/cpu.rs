use core::ptr::NonNull;
use super::ProcessUnit;


pub fn cpuid() -> usize {
    let id = tp_read();
    id
}

pub struct CPUManager {
    process: Option<NonNull<ProcessUnit>>,
}


pub struct CPU {

}

// read and write tp, the thread pointer, which holds
// this core's hartid (core number), the index into cpus[].
#[inline]
pub fn tp_read() -> usize {
    let ret: usize;
    unsafe {
        asm!(
            "mv {0}, tp",
            out(reg) ret
        );    
    }
    ret
}

#[inline]
pub fn tp_write(value: usize) {
    unsafe {
        asm!(
            "mv tp, {0}",
            in(reg) value
        );
    }
}