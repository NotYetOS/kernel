#![allow(unused)]

use alloc::string::String;
use alloc::vec::Vec;
use riscv::register::sstatus::SPP;
use crate::mm::MemorySet;

pub struct TaskUnit {
    pub satp: usize,
    mem_set: MemorySet,
}

impl TaskUnit {
    pub fn new(elf_data: &[u8]) -> Self {
        let set = MemorySet::from_elf(
            SPP::User, 
            &elf_data
        );

        Self {
            satp: set.satp_bits(),
            mem_set: set
        }
    }
}

impl Clone for TaskUnit {
    fn clone(&self) -> Self {
        let mem_set = self.mem_set.clone();
        Self {
            satp: mem_set.satp_bits(),
            mem_set,
        }
    }
}
