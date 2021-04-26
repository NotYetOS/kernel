mod handler;
mod timer;

pub fn enable() {
    use riscv::register::stvec::TrapMode;
    use riscv::register::stvec;

    extern "C" { fn _trap_entry(); }

    unsafe {
        stvec::write(
            _trap_entry as usize, 
            TrapMode::Direct
        );
    }
    timer::set_next_trigger();
    timer::enable();
}


pub fn get_satp() -> usize {
    use crate::config::CONTEXT;
    let mut satp: usize;
    
    unsafe {
        asm!(
            "ld {1}, 34*8({0})",
            in(reg) CONTEXT,
            out(reg) satp
        );
    }
    satp
}

pub fn get_kernel_satp() -> usize {
    use crate::config::CONTEXT;
    let mut satp: usize;
    
    unsafe {
        asm!(
            "ld {1}, 35*8({0})",
            in(reg) CONTEXT,
            out(reg) satp
        );
    }
    satp
}

#[allow(unused)]
pub fn test() {
    println!("");
    println!("[test] trap");
    println!("----------------------->");

    unsafe { asm! { "ebreak" } }

    println!("<-----------------------");
    println!("[passed] trap test");
}

pub use handler::trap_handler;
pub use timer::get_time_ms;
