// 没有操作系统的实现，所以禁用std
// 同样没有main
// 开启panic_info_message特性
// 开启global_asm特性

#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(global_asm)]

// 搞进entry.asm
global_asm!(include_str!("entry.asm"));

mod panic;

// 防止编译器瞎生成函数名
// 让entry.asm的call指令找得到main
#[no_mangle]
fn main() {
    loop {}
}