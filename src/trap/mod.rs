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

pub use context::TrapContext;
pub use handler::trap_handler;
