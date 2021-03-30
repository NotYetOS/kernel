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
// to support alloc error handler
#![feature(alloc_error_handler)]

// load entry.asm
global_asm!(include_str!("entry.s"));

#[macro_use]
mod console;
mod panic;
mod sbi;
mod mm;
mod config;
mod drivers;
mod fs;
mod trap;
mod process;
mod task;
mod syscall;

#[macro_use]
extern crate bitflags;
extern crate alloc;

#[no_mangle]
fn clear_bss() {
    extern "C" {
        fn estack();
        fn ebss();
    }

    (estack as usize..ebss as usize).for_each(|addr| 
        unsafe { 
            (addr as *mut u8).write_volatile(0) 
        }
    );
}

// prevent the compiler from blindly generating function names
// let the call command find main function
#[no_mangle]
fn main() {
    mm::init();
    trap::enable();

    println!("");
    println!("你好世界");
    println!("this is NotYetOS");

    trap::test();
    fs::test();
    process::test();

    println!("");
    println!("wow, i'm stupid");
    panic!("emm, to panic");
}
