#![allow(unused)]

// use marco to improve the code
macro_rules! impl_into {
    ($from: ty => usize) => {
        impl From<$from> for usize {
            fn from(f: $from) -> Self {
                f.0
            }
        }
    };
    (usize => $to: ty) => {
        impl From<usize> for $to {
            fn from(f: usize) -> Self {
                Self(f)
            }
        }
    };
    ($from: ty => $to: ty) => {
        impl From<$from> for $to {
            fn from(f: $from) -> Self {
                Self(f.0 << PAGE_SIZE_BITS)
            }
        }
    };
    ($from: ty => $to: ty; AddrToPageNum) => {
        impl From<$from> for $to {
            fn from(f: $from) -> Self {
                f.floor()
            }
        }
    }
}

use crate::config::*;
use super::PageTableEntry;

#[derive(Clone, Copy)]
pub struct VirtAddr(usize);

#[derive(Clone, Copy)]
pub struct VirtPageNum(usize);

#[derive(Clone, Copy)]
pub struct PhysAddr(usize);

#[derive(Clone, Copy)]
pub struct PhysPageNum(usize);

impl VirtAddr {
    pub fn floor(&self) -> VirtPageNum { 
        VirtPageNum(self.0 / PAGE_SIZE) 
    }
    
    pub fn ceil(&self) -> VirtPageNum { 
        VirtPageNum((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE) 
    }
}

impl VirtPageNum {
    pub fn indexes(&self) -> [usize; 3] {
        let mut vpn = self.0;
        let mut ret = [0; 3];
        for i in (0..3).rev() {
            ret[i] = vpn & 511;
            vpn >>= 9; 
        }
        ret
    }
}

impl PhysAddr {
    pub fn floor(&self) -> PhysPageNum { 
        PhysPageNum(self.0 / PAGE_SIZE) 
    }
    
    pub fn ceil(&self) -> PhysPageNum { 
        PhysPageNum((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE) 
    }
}

impl PhysPageNum {
    pub fn get_ptes(&self) -> &'static mut [PageTableEntry] {
        let pa: PhysAddr = self.clone().into();
        unsafe {
            core::slice::from_raw_parts_mut(pa.0 as *mut PageTableEntry, PAGE_SIZE / 8)
        }
    }

    pub fn get_page_bytes(&self) -> &'static mut [u8] {
        let pa: PhysAddr = self.clone().into();
        unsafe {
            core::slice::from_raw_parts_mut(pa.0 as *mut u8, PAGE_SIZE)
        }
    }

    pub fn get_mut<T>(&self) -> &'static mut T {
        let pa: PhysAddr = self.clone().into();
        unsafe {
            (pa.0 as *mut T).as_mut().unwrap()
        }
    }
}

impl_into!(VirtPageNum => VirtAddr);
impl_into!(PhysPageNum => PhysAddr);

impl_into!(VirtAddr => VirtPageNum; AddrToPageNum);
impl_into!(PhysAddr => PhysPageNum; AddrToPageNum);

impl_into!(usize => VirtAddr);
impl_into!(usize => VirtPageNum);
impl_into!(usize => PhysAddr);
impl_into!(usize => PhysPageNum);

impl_into!(VirtAddr => usize);
impl_into!(VirtPageNum => usize);
impl_into!(PhysAddr => usize);
impl_into!(PhysPageNum => usize);
