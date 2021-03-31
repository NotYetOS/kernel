#![allow(unused)]

use crate::config::*;
use crate::syscall::*;
use crate::mm::PageTable;
use crate::process;
use super::context::TrapContext;
use riscv::register::sstatus::{
    self,
    SPP
};
use riscv::register::{
    sepc, 
    stval
};
use riscv::register::scause::{
    Trap,
    Interrupt,
    Exception,
};
use riscv::register::scause::{
    self, 
    Scause
};

global_asm!(include_str!("trap.s"));

extern "C" { 
    fn _restore(); 
}

#[no_mangle]
pub fn trap_handler() {
    let scause = scause::read();

    let satp = super::get_satp();
    let pt = PageTable::from_satp(satp);
    let pa = pt.translate_va_to_pa(TRAP_CONTEXT.into()).unwrap();
    let cx = pa.get_mut::<TrapContext>();

    if scause.is_interrupt() {
        interrupt_handler(scause, cx);
    } else {
        exception_handler(scause, cx);
    }

    unsafe { _restore() };
}

fn interrupt_handler(cause: Scause, cx: &mut TrapContext) {
    let stval = stval::read();

    // if interrupt happened, sepc value is the next pc where the interrupt happened
    match cause.cause() {
        Trap::Interrupt(Interrupt::UserSoft) => {},
        Trap::Interrupt(Interrupt::SupervisorSoft) => {},
        Trap::Interrupt(Interrupt::UserTimer) => {},
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            super::timer::set_next_trigger();
            process::suspend();
        },
        Trap::Interrupt(Interrupt::UserExternal) => {},
        Trap::Interrupt(Interrupt::SupervisorExternal) => {},
        Trap::Interrupt(Interrupt::Unknown) => {},
        _ => unreachable!()
    }
}

fn exception_handler(cause: Scause, cx: &mut TrapContext) {
    // if exception happened, sepc value is the pc where the exception happened
    // ecall or ebreak instruction is 4 byte
    cx.sepc += 4;

    match cause.cause() {
        Trap::Exception(Exception::InstructionMisaligned) => {},
        Trap::Exception(Exception::InstructionFault) => {},
        Trap::Exception(Exception::IllegalInstruction) => {},
        Trap::Exception(Exception::Breakpoint) => println!("ebreak"),  
        Trap::Exception(Exception::LoadFault) => {},
        Trap::Exception(Exception::StoreMisaligned) => {},
        Trap::Exception(Exception::StoreFault) => {},
        Trap::Exception(Exception::UserEnvCall) => {
            let result = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]);
            cx.x[10] = result as usize;
        },
        Trap::Exception(Exception::InstructionPageFault) => {},
        Trap::Exception(Exception::LoadPageFault) => {},
        Trap::Exception(Exception::StorePageFault) => {},
        Trap::Exception(Exception::Unknown) => {},
        _ => unreachable!()
    }
}
