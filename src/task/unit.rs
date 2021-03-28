#![allow(unused)]

use alloc::string::String;
use alloc::vec::Vec;
use riscv::register::sstatus::SPP;
use crate::mm::MemorySet;
use crate::fs::ROOT;

pub struct TaskUnit {
    path: String,
    pub satp: usize,
    pub entry: usize,
    pub stack_top: usize,
    pub mode: usize,
}

impl TaskUnit {
    pub fn new(path: &str) -> Self {
        let bin_dir = ROOT.lock().cd("bin").unwrap();
        let bin = bin_dir.open_file(&path).unwrap();
        let mut elf_data = Vec::new();
        let len = bin.read_to_vec(&mut elf_data).unwrap();
        
        let (
            set, 
            stack_top, 
            entry,
            mode
        ) = MemorySet::from_elf(SPP::User, &elf_data[0..len]);

        Self {
            path: path.into(),
            entry,
            stack_top,
            satp: set.satp_bits(),
            mode
        }
    }
}
