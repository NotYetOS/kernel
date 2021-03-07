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

impl PhysAddr {
    pub fn floor(&self) -> PhysPageNum { 
        PhysPageNum(self.0 / PAGE_SIZE) 
    }
    
    pub fn ceil(&self) -> PhysPageNum { 
        PhysPageNum((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE) 
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
