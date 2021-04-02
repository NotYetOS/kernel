#![allow(unused)]

// use marco to improve the code
macro_rules! impl_debug {
    ($target: ty) => {
        impl Debug for $target {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_fmt(format_args!("{}: {:#x}", stringify!($target), self.0))
            }
        }
    };
}

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

use core::fmt::{
    self, 
    Formatter,
    Debug
};

use core::ops::{
    Add, 
    Sub,
    AddAssign, 
    SubAssign
};

use crate::config::*;
use super::PageTableEntry;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VirtAddr(usize);

impl VirtAddr {
    pub fn value(&self) -> usize {
        self.0
    }

    pub fn floor(&self) -> VirtPageNum { 
        VirtPageNum(self.0 / PAGE_SIZE) 
    }
    
    // if VirtAddr % PAGE_SIZE == 0
    // VirtPageNum = VirtAddr / PAGE_SIZE
    // else
    // VirtPageNum = VirtAddr / PAGE_SIZE + 1
    pub fn ceil(&self) -> VirtPageNum { 
        if self.0 % PAGE_SIZE == 0 {
            self.floor()
        } else {
            self.floor() + 1
        }
    }

    pub fn page_offset(&self) -> usize { 
        self.0 & (PAGE_SIZE - 1) 
    }

    pub fn aligned(&self) -> bool { 
        self.page_offset() == 0
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[repr(C)]
pub struct VirtPageNum(usize);

impl VirtPageNum {
    pub fn new(value: usize) -> Self {
        Self(value)
    }

    pub fn indexes(&self) -> [usize; 3] {
        let mut vpn = self.0;
        let mut ret = [0; 3];
        for i in (0..3).rev() {
            ret[i] = vpn & 511;
            vpn >>= 9; 
        }
        ret
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysAddr(usize);

impl PhysAddr {
    pub fn value(&self) -> usize {
        self.0
    }

    pub fn floor(&self) -> PhysPageNum { 
        PhysPageNum(self.0 / PAGE_SIZE) 
    }
    
    // if PhysAddr % PAGE_SIZE == 0
    // PhysPageNum = PhysAddr / PAGE_SIZE
    // else
    // PhysPageNum = PhysAddr / PAGE_SIZE + 1
    pub fn ceil(&self) -> PhysPageNum { 
        if self.0 % PAGE_SIZE == 0 {
            self.floor()
        } else {
            self.floor() + 1
        }
    }

    pub fn page_offset(&self) -> usize { 
        self.0 & (PAGE_SIZE - 1) 
    }

    pub fn aligned(&self) -> bool { 
        self.page_offset() == 0
    }

    pub fn get_mut<T>(&self) -> &'static mut T {
        unsafe {
            (self.0 as *mut T).as_mut().unwrap()
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[repr(C)]
pub struct PhysPageNum(usize);

impl PhysPageNum {
    pub fn get_ptes(&self) -> &'static mut [PageTableEntry] {
        let pa: PhysAddr = self.clone().into();
        unsafe {
            core::slice::from_raw_parts_mut(
                pa.0 as *mut PageTableEntry, 
                PAGE_SIZE / 8
            )
        }
    }

    pub fn get_page_bytes(&self) -> &'static mut [u8] {
        let pa: PhysAddr = self.clone().into();
        unsafe {
            core::slice::from_raw_parts_mut(
                pa.0 as *mut u8, 
                PAGE_SIZE
            )
        }
    }

    pub fn get_mut<T>(&self) -> &'static mut T {
        let pa: PhysAddr = self.clone().into();
        pa.get_mut::<T>()
    }

    pub fn clean(&self) {
        self.get_page_bytes().copy_from_slice(
            &[0; PAGE_SIZE]
        );
    }
}

impl Add<usize> for PhysPageNum {
    type Output = PhysPageNum;

    fn add(self, rhs: usize) -> Self::Output {
        (self.0 + rhs).into()
    }
}

impl Sub<usize> for PhysPageNum {
    type Output = PhysPageNum;

    fn sub(self, rhs: usize) -> Self::Output {
        (self.0 - rhs).into()
    }
}

impl AddAssign<usize> for PhysPageNum {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl SubAssign<usize> for PhysPageNum {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs;
    }
}

impl Add<usize> for VirtPageNum {
    type Output = VirtPageNum;

    fn add(self, rhs: usize) -> Self::Output {
        (self.0 + rhs).into()
    }
}

impl Sub<usize> for VirtPageNum {
    type Output = VirtPageNum;

    fn sub(self, rhs: usize) -> Self::Output {
        (self.0 - rhs).into()
    }
}

impl AddAssign<usize> for VirtPageNum {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl SubAssign<usize> for VirtPageNum {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs;
    }
}

#[derive(Clone, Copy)]
pub struct VPNRange {
    start: VirtPageNum,
    current: VirtPageNum,
    end: VirtPageNum, 
}

impl VPNRange {
    pub fn new(
        start: VirtPageNum, 
        end: VirtPageNum
    ) -> Self {
        Self {
            start,
            current: start,
            end,
        }
    }

    pub fn current(&self) -> VirtPageNum {
        self.current
    }

    pub fn get_start(&self) -> VirtPageNum {
        self.start
    }

    pub fn get_end(&self) -> VirtPageNum {
        self.end
    }
}

impl Iterator for VPNRange {
    type Item = VirtPageNum;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            self.current += 1;
            Some(self.current - 1)
        } else {
            None
        }
    }
}

impl_debug!(VirtAddr);
impl_debug!(VirtPageNum);
impl_debug!(PhysAddr);
impl_debug!(PhysPageNum);

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
