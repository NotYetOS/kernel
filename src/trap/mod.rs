mod handler;
mod context;

pub fn enable() {
    use riscv::register::stvec::TrapMode;
    use riscv::register::stvec;
    use crate::config::*;

    extern "C" { fn _trap_entry(); }

    println!("{:#x}", _trap_entry as usize);

    unsafe {
        stvec::write(TRAMPOLINE, TrapMode::Direct);
    }
}

pub fn get_satp() -> usize {
    let mut satp: usize;
    unsafe {
        asm!(
            "
            li t1, 0xffffffffffffe000
            ld t2, 34*8(t1)
            ",
            lateout("t2") satp
        )
    }
    satp
}

pub use context::TrapContext;
pub use handler::trap_handler;
