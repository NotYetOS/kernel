#![allow(unused)]

use crate::config::*;

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

#[no_mangle]
pub fn trap_handler() {
    let scause = scause::read();
    println!("{:?}", scause.cause());
    panic!();
    let mut cx = unsafe {
        (TRAP_CONTEXT as *mut TrapContext).as_mut().unwrap()
    };

    if scause.is_interrupt() {
        interrupt_handler(scause, cx);
    } else {
        exception_handler(scause, cx);
    }
}

fn interrupt_handler(cause: Scause, cx: &mut TrapContext) {
    let stval = stval::read();

    // if interrupt happened, sepc value is the next pc where the interrupt happened
    match cause.cause() {
        Trap::Interrupt(Interrupt::UserSoft) => {},
        Trap::Interrupt(Interrupt::SupervisorSoft) => {},
        Trap::Interrupt(Interrupt::UserTimer) => {},
        Trap::Interrupt(Interrupt::SupervisorTimer) => {},
        Trap::Interrupt(Interrupt::UserExternal) => {},
        Trap::Interrupt(Interrupt::SupervisorExternal) => {},
        Trap::Interrupt(Interrupt::Unknown) => {},
        _ => unreachable!()
    }
}

fn exception_handler(cause: Scause, cx: &mut TrapContext) {
    let sepc_value = sepc::read();
    // if exception happened, sepc value is the pc where the exception happened
    // ecall or ebreak instruction is 4 byte
    cx.sepc += 4;

    match cause.cause() {
        Trap::Exception(Exception::InstructionMisaligned) => {},
        Trap::Exception(Exception::InstructionFault) => {},
        Trap::Exception(Exception::IllegalInstruction) => {},
        Trap::Exception(Exception::Breakpoint) => {
            println!("hello user ebreak");
            println!("ready switch from user to supervisor");
            let mut sstatus = sstatus::read();
            sstatus.set_spp(SPP::Supervisor);
            cx.sstatus = sstatus;
        },  
        Trap::Exception(Exception::LoadFault) => {},
        Trap::Exception(Exception::StoreMisaligned) => {},
        Trap::Exception(Exception::StoreFault) => {},
        Trap::Exception(Exception::UserEnvCall) => {},
        Trap::Exception(Exception::InstructionPageFault) => {},
        Trap::Exception(Exception::LoadPageFault) => {},
        Trap::Exception(Exception::StorePageFault) => {},
        Trap::Exception(Exception::Unknown) => {},
        _ => unreachable!()
    }
}
