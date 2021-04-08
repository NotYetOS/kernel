#![allow(unused)]

use crate::config::*;
use crate::syscall::*;
use crate::mm::PageTable;
use crate::process;
use crate::context::{
    Context, 
    get_context
};
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
    let stval = stval::read();
    let cx = get_context(
        super::get_satp()
    );
 
    if scause.is_interrupt() {
        interrupt_handler(scause, cx);
    } else if scause.is_exception() {
        exception_handler(scause, cx);
    } else {
        panic!("unsupport trap {:?}, stval = {:#x}", scause.cause(), stval);
    }

    unsafe { _restore() };
}

fn interrupt_handler(cause: Scause, cx: &mut Context) {
    let stval = stval::read();

    // if interrupt happened, sepc value is the next pc where the interrupt happened
    match cause.cause() {
        Trap::Interrupt(Interrupt::UserSoft) => {},
        Trap::Interrupt(Interrupt::SupervisorSoft) => {},
        Trap::Interrupt(Interrupt::UserTimer) => {},
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            super::timer::set_next_trigger();
            process::suspend();
            process::ret();
        },
        Trap::Interrupt(Interrupt::UserExternal) => {},
        Trap::Interrupt(Interrupt::SupervisorExternal) => {},
        Trap::Interrupt(Interrupt::Unknown) => {},
        _ => unreachable!()
    }
}

fn exception_handler(cause: Scause, cx: &mut Context) {
    // if exception happened, sepc value is the pc where the exception happened
    // ecall or ebreak instruction is 4 byte
    cx.sepc += 4;

    match cause.cause() {
        Trap::Exception(Exception::InstructionMisaligned) => {},
        Trap::Exception(Exception::InstructionFault) => {},
        Trap::Exception(Exception::IllegalInstruction) => {},
        Trap::Exception(Exception::Breakpoint) => {},  
        Trap::Exception(Exception::LoadFault) => {},
        Trap::Exception(Exception::StoreMisaligned) => {},
        Trap::Exception(Exception::StoreFault) => {},
        Trap::Exception(Exception::UserEnvCall) => {
            let id = cx.x[17];
            cx.x[10] = syscall(
                id, 
                [cx.x[10], cx.x[11], cx.x[12]]
            ) as usize;

            if is_process_call(id) {
                process::ret();
            }        
        },
        Trap::Exception(Exception::InstructionPageFault) => {},
        Trap::Exception(Exception::LoadPageFault) => {},
        Trap::Exception(Exception::StorePageFault) => {},
        Trap::Exception(Exception::Unknown) => {},
        _ => unreachable!()
    }
}
