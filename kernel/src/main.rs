// 没有操作系统的实现，所以禁用std
#![no_std]
// 同样没有main
#![no_main]
// 开启panic_info_message特性
#![feature(panic_info_message)]
// 开启global_asm特性
#![feature(global_asm)]
// 开启内联汇编特性
#![feature(llvm_asm)]
// 开启arbitrary_enum_discriminant，是个好东西
// 具体可看sbi/ret的ErrorType
#![feature(arbitrary_enum_discriminant)]

// 搞进entry.asm
global_asm!(include_str!("entry.asm"));

mod panic;
mod sbi;

// 防止编译器瞎生成函数名
// 让entry.asm的call指令找得到main
#[no_mangle]
fn main() {
    panic!();
}