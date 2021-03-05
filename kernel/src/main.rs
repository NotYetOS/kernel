// 没有操作系统的实现，所以禁用std
// 同样没有main
// 开启panic info message特性
#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod panic;