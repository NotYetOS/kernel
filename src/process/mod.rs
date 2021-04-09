#![allow(unused)]

mod unit;
mod manager;
mod pid;

pub fn test() {
    println!("");
    println!("[test] process");
    println!("----------------------->");
    
    exec("shell");
    run();

    println!("<-----------------------");
    println!("[passed] process test");
}

pub use manager::*;
