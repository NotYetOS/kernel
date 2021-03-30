#![allow(unused)]

use alloc::string::String;
use alloc::vec::Vec;
use riscv::register::sstatus::SPP;
use crate::mm::MemorySet;

pub struct TaskUnit {
    pub satp: usize,
    pub entry: usize,
    pub stack_top: usize,
    pub mode: usize,
    mem_set: MemorySet,
}

impl TaskUnit {
    pub fn new(elf_data: &[u8]) -> Self {
        let (
            set, 
            stack_top, 
            entry,
            mode
        ) = MemorySet::from_elf(SPP::User, &elf_data);

        Self {
            entry,
            stack_top,
            satp: set.satp_bits(),
            mode,
            mem_set: set
        }
    }
}
