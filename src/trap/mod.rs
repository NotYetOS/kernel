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
    let mut satp: usize;
    unsafe {
        asm!(
            "
            li t1, 0xfffffffffffff000
            ld t2, 34*8(t1)
            ",
            lateout("t2") satp
        )
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
