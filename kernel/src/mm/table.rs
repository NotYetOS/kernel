#![allow(unused)]

use super::{
    FrameTracker, 
    PageTableEntry,
    PhysPageNum,
    VirtPageNum, 
    PTEFlags,
    frame_alloc,
};
use alloc::vec::Vec;

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
    frames: Vec<FrameTracker>
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

    fn find_pte_by_create(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let indexes = vpn.indexes();
        let mut ppn = self.root;
        for &i in &indexes[0..2] {
            let pte = &mut ppn.get_ptes()[i];
            if !pte.is_valid() {
                let f = frame_alloc();
                *pte = PageTableEntry::new(f.clone().into(), PTEFlags::V);
                self.frames.push(f);
            }
            ppn = pte.ppn();
        }
        Some(&mut ppn.get_ptes()[indexes[2]])
    }

    fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum) {
        let pte = match self.find_pte_by_create(vpn) {
            Some(pte) => pte,
            None => unreachable!()
        };
        assert!(!pte.is_valid(), "{:?} is invalid before unmapping", vpn);
        *pte = PageTableEntry::new(ppn, PTEFlags::V);
    }

    fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = match self.find_pte_by_create(vpn) {
            Some(pte) => pte,
            None => unreachable!()
        };
        assert!(pte.is_valid(), "{:?} is invalid before unmapping", vpn);
        *pte = PageTableEntry::empty();
    }
}
