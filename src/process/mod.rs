#![allow(unused)]

mod unit;
mod manager;
mod pid;

use alloc::vec;

pub fn start() {
    println!("");
    println!("[test] process");
    println!("----------------------->");
    
    exec("shell", vec![]);
    run();

    println!("<-----------------------");
    println!("[passed] process test");
}

pub use manager::*;
