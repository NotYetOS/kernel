// there is no operating system implementation, so std and main is disabled
#![no_std]
#![no_main]
// to support panic handler
#![feature(panic_info_message)]
// to support module-level inline assembly
#![feature(global_asm)]
// to support inline assembly
#![feature(asm)]
// see sbi/ret/ErrorType
#![feature(arbitrary_enum_discriminant)]

// load entry.asm
global_asm!(include_str!("entry.asm"));

#[macro_use]
mod console;
mod panic;
mod sbi;
mod mm;
mod config;

#[macro_use]
extern crate bitflags;

// prevent the compiler from blindly generating function names
// let the call command find main function
#[no_mangle]
fn main() {
    println!("wow, i'm stupid");
    panic!("emm, to panic");
}
