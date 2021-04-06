#![allow(unused)]

mod unit;
mod manager;
mod pid;

pub fn test() {
    use crate::task::TaskUnit;
    use crate::fs::ROOT;
    use unit::ProcessUnit;
    use fefs::inode::INodeType;
    use alloc::vec::Vec;
    use alloc::sync::Arc;
    
    println!("");
    println!("[test] process");
    println!("----------------------->");
    
    exec("shell");
    run();

    println!("<-----------------------");
    println!("[passed] process test");
}

pub use manager::*;
