#![allow(unused)]

bitflags! {
    pub struct PTEFlags: u8 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}

use super::PhysPageNum;

// PTE is 8 bytes
// so 512 * 8 = 4096 = 4K = PAGE_SIZE
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PageTableEntry {
    bits: usize,
}

impl PageTableEntry {
    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        let bits: usize = (usize::from(ppn) << 10) | flags.bits as usize;
        Self { bits }
    }

    pub fn ppn(&self) -> PhysPageNum {
        (self.bits >> 10 & (1 << 44) - 1).into()
    }

    pub fn empty() -> Self {
        PageTableEntry { bits: 0 }
    }

    pub fn flags(&self) -> PTEFlags {
        // cut 8 bits from low
        PTEFlags::from_bits(self.bits as u8).unwrap()
    }

    pub fn is_valid(&self) -> bool {
        (self.flags() & PTEFlags::V) != PTEFlags::empty()
    }

    pub fn readable(&self) -> bool {
        (self.flags() & PTEFlags::R) != PTEFlags::empty()
    }

    pub fn writable(&self) -> bool {
        (self.flags() & PTEFlags::W) != PTEFlags::empty()
    }

    pub fn executable(&self) -> bool {
        (self.flags() & PTEFlags::X) != PTEFlags::empty()
    }

    pub fn is_user_page(&self) -> bool {
        (self.flags() & PTEFlags::X) != PTEFlags::empty()
    }

    pub fn is_supervisor_page(&self) -> bool {
        !self.is_user_page()
    }

    pub fn is_valid_for_all_virt_space(&self) -> bool {
        (self.flags() & PTEFlags::G) != PTEFlags::empty()
    }

    pub fn is_visited_after_a_be_cleaned(&self) -> bool {
        (self.flags() & PTEFlags::A) != PTEFlags::empty()
    }

    pub fn is_dirty_after_d_be_cleaned(&self) -> bool {
        (self.flags() & PTEFlags::D) != PTEFlags::empty()
    }

    pub fn is_leaf(&self) -> bool {
        self.readable() | self.writable() | self.executable()
    }

    pub fn is_node(&self) -> bool {
        !self.is_leaf()
    }
}
