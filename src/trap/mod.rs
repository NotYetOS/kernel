mod handler;
mod context;

pub fn enable() {
    use riscv::register::stvec::TrapMode;
    use riscv::register::stvec;

    extern "C" { fn _trap_entry(); }

    unsafe {
        stvec::write(_trap_entry as usize, TrapMode::Direct);
    }
}

pub fn get_satp() -> usize {
    use crate::config::TRAP_CONTEXT;
    let mut satp: usize;
    
    unsafe {
        asm!(
            "ld {1}, 34*8({0})",
            in(reg) TRAP_CONTEXT,
            out(reg) satp
        );
    }
    satp
}

pub fn test() {
    println!("");
    println!("[test] trap");
    println!("----------------------->");

    unsafe { asm! { "ebreak" } }

    println!("<-----------------------");
    println!("[passed] trap test");
}

pub use context::TrapContext;
pub use handler::trap_handler;
