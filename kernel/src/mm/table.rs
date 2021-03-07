#![allow(unused)]

use super::{
    PhysPageNum,
    VirtPageNum,
    PageTableEntry,
};

// define mode, Sv32 not impl, just look...
pub enum Mode {
    #[allow(unused)]
    Bare = 0,
    #[allow(unused)]
    Sv32 = 1,
    Sv39 = 8
}

struct PageTable {
    root: PhysPageNum,
}

impl PageTable {
    fn satp_bits(&self, mode: Mode) -> usize {
        (mode as usize) << 60 | usize::from(self.root)
    }

    fn find_pte(&self, vpn: VirtPageNum) -> Option<&PageTableEntry> {
        let indexes = vpn.indexes();
        let mut ppn = self.root;
        for &i in &indexes[0..2] {
            let pte = &mut ppn.get_ptes()[i];
            if !pte.is_valid() { return None; }
            ppn = pte.ppn();
        }
        Some(&ppn.get_ptes()[indexes[2]])
    }
}