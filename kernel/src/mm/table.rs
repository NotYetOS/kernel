#![allow(unused)]

use super::{
    FrameTracker, 
    PTEFlags, 
    PageTableEntry, 
    PhysAddr, 
    PhysPageNum, 
    VirtAddr, 
    VirtPageNum, 
    frame_alloc
};
use alloc::{
    vec,
    vec::Vec
};

// define mode, Sv32 not impl, just look...
pub enum Mode {
    #[allow(unused)]
    Bare = 0,
    #[allow(unused)]
    Sv32 = 1,
    Sv39 = 8
}

pub struct PageTable {
    root: PhysPageNum,
    frames: Vec<FrameTracker>
}

impl PageTable {
    pub fn new() -> Self {
        let frame = frame_alloc();
        Self {
            root: frame.ppn(),
            frames: vec![frame]
        }
    }

    pub fn from_satp(satp: usize) -> Self {
        Self {
            root: PhysPageNum::from(satp & ((1usize << 44) - 1)),
            frames: Vec::new(),
        }
    }

    pub fn satp_bits(&self, mode: Mode) -> usize {
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
                *pte = PageTableEntry::new(f.ppn(), PTEFlags::V);
                self.frames.push(f);
            }
            ppn = pte.ppn();
        }
        Some(&mut ppn.get_ptes()[indexes[2]])
    }

    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.find_pte(vpn).map(|pte| pte.clone())
    }

    pub fn translate_va_to_pa(&self, va: VirtAddr) -> Option<PhysAddr> {
        self.find_pte(va.into()).map(|pte| {
            let aligned_pa: PhysAddr = pte.ppn().into();
            let offset = va.page_offset();
            let aligned_pa_usize: usize = aligned_pa.into();
            (aligned_pa_usize + offset).into()
        })
    }

    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let pte = match self.find_pte_by_create(vpn) {
            Some(pte) => pte,
            None => unreachable!()
        };
        assert!(!pte.is_valid(), "{:?} is invalid before unmapping", vpn);
        *pte = PageTableEntry::new(ppn, flags | PTEFlags::V);
    }

    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = match self.find_pte_by_create(vpn) {
            Some(pte) => pte,
            None => unreachable!()
        };
        assert!(pte.is_valid(), "{:?} is invalid before unmapping", vpn);
        *pte = PageTableEntry::empty();
    }
}
