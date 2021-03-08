#![allow(unused)]

use alloc::{
    collections::BTreeMap,
    vec::Vec
};
use super::{
    FrameTracker, 
    Mode, 
    PTEFlags, 
    PageTable, 
    VPNRange, 
    VirtAddr, 
    VirtPageNum, 
    frame_alloc
};
use riscv::register::satp;
use crate::config::*;

bitflags! {
    pub struct MapPermission: u8 {
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum MapType {
    Identical,
    Alloc,
}

pub struct MemorySet {
    table: PageTable,
    areas: Vec<MapArea>
}

impl MemorySet {
    pub fn new() -> Self {
        Self {
            table: PageTable::new(),
            areas: Vec::new()
        }
    }

    fn push(&mut self, mut area: MapArea, data: Option<&[u8]>) {
        area.map(&mut self.table);
        if let Some(data) = data {
            area.copy_data(&mut self.table, data);
        }
        self.areas.push(area);
    }

    pub fn new_kernel() -> Self {
        let mut set = MemorySet::new();

        extern "C" {
            fn stext();
            fn etext();
            fn srodata();
            fn erodata();
            fn sdata();
            fn edata();
            fn sstack();
            fn estack();
            fn sbss();
            fn ebss();
            fn ekernel();
        }

        println!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
        println!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
        println!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
        println!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);

        println!("mapping .text section");
        set.push(
            MapArea::new(
                (stext as usize).into(),
                (etext as usize).into(), 
                MapType::Identical, 
                MapPermission::R | MapPermission::X
            ), None
        );

        println!("mapping .rodata section");
        set.push(
            MapArea::new(
                (srodata as usize).into(),
                (erodata as usize).into(), 
                MapType::Identical, 
                MapPermission::R
            ), None
        );

        println!("mapping .data section");
        set.push(
            MapArea::new(
                (sdata as usize).into(),
                (edata as usize).into(), 
                MapType::Identical, 
                MapPermission::R | MapPermission::W
            ), None
        );

        println!("mapping .bss section");
        set.push(
            MapArea::new(
                (sbss as usize).into(),
                (ebss as usize).into(), 
                MapType::Identical, 
                MapPermission::R | MapPermission::W
            ), None
        );

        println!("mapping physical memory");
        set.push(
            MapArea::new(
                (ekernel as usize).into(),
                MEMORY_END.into(), 
                MapType::Identical, 
                MapPermission::R | MapPermission::W
            ), None
        );

        set
    }

    // activate Sv39
    pub fn activate(&self, mode: Mode) {
        let satp = self.table.satp_bits(mode);
        unsafe {
            satp::write(satp);
            asm!(
                "sfence.vma",
                options(nostack)
            );
        }
    }
}

struct MapArea {
    range: VPNRange,
    mtype: MapType,
    allocated: BTreeMap<VirtPageNum, FrameTracker>,
    permission: MapPermission,
}

impl MapArea {
    pub fn new(
        start: VirtAddr,
        end: VirtAddr,
        mtype: MapType,
        permission: MapPermission
    ) -> Self {
        Self {
            range: VPNRange::new(start.floor(), end.ceil()),
            mtype,
            allocated: BTreeMap::new(),
            permission
        }
    }

    pub fn map(&mut self, table: &mut PageTable) {
        match self.mtype {
            MapType::Identical => self.map_identical(table),
            MapType::Alloc => self.map_alloc(table),
        }
    }

    fn map_identical(&self, table: &mut PageTable) {
        let flags = PTEFlags::from_bits(self.permission.bits()).unwrap();

        for vpn in self.range {
            table.map(vpn, vpn.value().into(), flags)
        }
    }

    fn map_alloc(&mut self, table: &mut PageTable) {
        let flags = PTEFlags::from_bits(self.permission.bits()).unwrap();

        for vpn in self.range {
            let frame = frame_alloc();
            table.map(vpn, frame.ppn(),flags);
            self.allocated.insert(vpn, frame);
        }
    }

    fn copy_data(&mut self, table: &mut PageTable, data: &[u8]) {
        assert_eq!(self.mtype, MapType::Alloc);
        let mut start: usize = 0;
        let mut current_vpn = self.range.current();
        let len = data.len();
        loop {
            let src = &data[start..len.min(start + PAGE_SIZE)];
            let dst = &mut table.translate(current_vpn)
                .unwrap()
                .ppn()
                .get_page_bytes()[..src.len()];
            dst.copy_from_slice(src);
            start += PAGE_SIZE;
            if start >= len {
                break;
            }
            current_vpn += 1;
        }
    }
}
