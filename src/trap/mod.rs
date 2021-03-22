mod handler;
mod context;

pub fn enable() {
    use riscv::register::stvec::TrapMode;
    use riscv::register::stvec;

    extern "C" { fn _entry(); }

    unsafe {
        stvec::write(_entry as usize, TrapMode::Direct);
    }
}

pub fn user_mode_trap_test() {
    use super::trap::context::TrapContext;
    use riscv::register::sstatus::SPP;

    let sp = [0; 5120];

    let cx = TrapContext::app_trap_context(
        SPP::User,
        ebreak_test as usize, 
        sp.as_ptr() as usize + sp.len()
    );

    println!("");
    println!("this is user mode trap test");

    extern "C" {
        fn _restore(cx: usize);
    }

    unsafe {
        _restore(&cx as *const _ as usize);
    }
}

#[allow(unused)]
fn ebreak_test() {
    unsafe {
        asm!(
            "ebreak",
            options(nostack)
        )
    }
    
    println!("hello supervisor");
    println!("trap test passed");
    crate::then();
}
