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
    
    let mut elf_data = Vec::new();
    let bin_dir = ROOT.lock().cd("bin").unwrap();
    for node in bin_dir.ls() {
        match node.inode_type() {
            INodeType::FileEntry => {
                let bin = bin_dir.open_file(&node.name()).unwrap();
                let len = bin.read_to_vec(&mut elf_data).unwrap();
                let task = TaskUnit::new(&elf_data[0..len]);
                let process = ProcessUnit::new(task);
                push_process(Arc::new(process));
            }
            _ => {}
        }
    }

    run();
    println!("<-----------------------");
    println!("[passed] process test");
}

pub use manager::*;
