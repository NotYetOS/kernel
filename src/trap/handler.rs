#![allow(unused)]

use crate::config::*;
use crate::context::{get_context, Context};
use crate::mm::PageTable;
use crate::process;
use crate::syscall::*;
use riscv::register::scause::{self, Scause};
use riscv::register::scause::{Exception, Interrupt, Trap};
use riscv::register::sstatus::{self, SPP};
use riscv::register::{sepc, stval};

global_asm!(include_str!("trap.s"));

extern "C" {
    fn _restore();
}

#[no_mangle]
pub fn trap_handler() {
    let scause = scause::read();
    let stval = stval::read();
    let cx = get_context(super::get_satp());

    if scause.is_interrupt() {
        interrupt_handler(scause, cx, stval);
    } else if scause.is_exception() {
        exception_handler(scause, cx, stval);
    } else {
        panic!("unsupport trap {:?}, stval = {:#x}", scause.cause(), stval);
    }

    unsafe { _restore() };
}

fn interrupt_handler(cause: Scause, cx: &mut Context, stval: usize) {
    let stval = stval::read();

    // if interrupt happened, sepc value is the next pc where the interrupt happened
    match cause.cause() {
        Trap::Interrupt(Interrupt::UserSoft) => {}
        Trap::Interrupt(Interrupt::SupervisorSoft) => {}
        Trap::Interrupt(Interrupt::UserTimer) => {}
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            super::timer::set_next_trigger();
            process::suspend();
        }
        Trap::Interrupt(Interrupt::UserExternal) => {}
        Trap::Interrupt(Interrupt::SupervisorExternal) => {}
        Trap::Interrupt(Interrupt::Unknown) => {}
        _ => unreachable!(),
    }
}

fn exception_handler(cause: Scause, cx: &mut Context, stval: usize) {
    // if exception happened, sepc value is the pc where the exception happened
    // ecall or ebreak instruction is 4 byte
    cx.sepc += 4;

    match cause.cause() {
        Trap::Exception(Exception::InstructionMisaligned) => {}
        Trap::Exception(Exception::InstructionFault) => {}
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, core dumped.");
            process::exit(-3);
            process::ret();
        }
        Trap::Exception(Exception::Breakpoint) => {}
        Trap::Exception(Exception::LoadFault) => {}
        Trap::Exception(Exception::StoreMisaligned) => {}
        Trap::Exception(Exception::StoreFault) => {}
        Trap::Exception(Exception::UserEnvCall) => {
            let id = cx.a7;
            cx.a0 = syscall(id, [cx.a0, cx.a1, cx.a2]) as usize;
        }
        Trap::Exception(Exception::InstructionPageFault) => {}
        Trap::Exception(Exception::LoadPageFault) => {
            println!(
                "[kernel] {:?} in application, bad addr = {:#x}, bad instruction = {:#x}, core dumped.",
                cause.cause(),
                stval,
                cx.sepc,
            );
            process::exit(-2);
        }
        Trap::Exception(Exception::StorePageFault) => {}
        Trap::Exception(Exception::Unknown) => {}
        _ => unreachable!(),
    }
}
