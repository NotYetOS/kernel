#![allow(unused)]

use core::ops::Deref;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use riscv::register::satp;
use riscv::register::sstatus::SPP;
use xmas_elf::ElfFile;
use xmas_elf::program;
use crate::config::*;
use crate::context::Context;
use super::{
    FrameTracker, 
    Mode, 
    PTEFlags, 
    PageTable, 
    PhysAddr, 
    VPNRange,
    VirtAddr, 
    VirtPageNum, 
    frame_alloc
};

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
    fn sasm();
    fn easm();
}

bitflags! {
    pub struct MapPermission: u8 {
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
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

    pub fn satp_bits(&self) -> usize {
        self.table.satp_bits(Mode::Sv39)
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

        println!("");
        println!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
        println!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
        println!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
        println!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);

        println!("mapping mmio");
        for &(start, size) in MMIO {
            set.push(MapArea::new(
                (start).into(), 
                (start + size).into(), 
                MapType::Identical, 
                MapPermission::R | MapPermission::W
            ), None);
        }

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

        let context = Context::init_context(
            SPP::Supervisor, 
            0, 
            0,
            0,
            set.satp_bits(),
            0,
            crate::trap::trap_handler as usize,
        );

        let context_bytes = unsafe {
            core::slice::from_raw_parts(
                &context as *const _ as *const u8, 
                core::mem::size_of::<Context>()
            )
        };

        set.push(MapArea::new(
            SUB_CONTEXT.into(),
            CONTEXT.into(),
            MapType::Alloc,
            MapPermission::R | MapPermission::W,
        ), Some(context_bytes));

        set.push(MapArea::new(
            CONTEXT.into(),
            usize::MAX.into(),
            MapType::Alloc,
            MapPermission::R | MapPermission::W,
        ), Some(context_bytes));

        set
    }

    pub fn from_elf(mode: SPP, data: &[u8]) -> Self {
        let mut set = MemorySet::new();

        set.push(
            MapArea::new(
                (sasm as usize).into(),
                (easm as usize).into(), 
                MapType::Identical, 
                MapPermission::R | MapPermission::X
            ), None
        );

        let elf = ElfFile::new(data).unwrap();
        let elf_header_part1 = elf.header.pt1;
        let elf_header_part2 = elf.header.pt2;
        assert_eq!(elf_header_part1.magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");

        let program_header_count = elf_header_part2.ph_count();
        let mut end_vpn = VirtPageNum::new(0);

        for idx in 0..program_header_count {
            let program_header = elf.program_header(idx).unwrap();
            match program_header.get_type() {
                Ok(program::Type::Load) => {
                    let start_va: VirtAddr = (program_header.virtual_addr() as usize).into();
                    let end_va: VirtAddr = (start_va.value() + program_header.mem_size() as usize).into();
                    let mut permission = MapPermission::U;
                    let program_flags = program_header.flags();
                    if program_flags.is_read() { permission |= MapPermission::R };
                    if program_flags.is_write() { permission |= MapPermission::W };
                    if program_flags.is_execute() { permission |= MapPermission::X };

                    let area = MapArea::new(
                        start_va,
                        end_va,
                        MapType::Alloc,
                        permission,
                    );

                    end_vpn = area.range.get_end();
                    let start = program_header.offset() as usize;
                    let end = start + program_header.file_size() as usize;
                    set.push(
                        area, 
                        Some(&elf.input[start..end])
                    );
                }
                _ => { /* no need to impl */ }
            }
        }

        let end_va: VirtAddr = end_vpn.into();
        let mut user_stack_bottom = end_va.value();
        user_stack_bottom += PAGE_SIZE;

        let user_stack_top = user_stack_bottom + USER_STACK_SIZE;

        set.push(MapArea::new(
            user_stack_bottom.into(),
            user_stack_top.into(),
            MapType::Alloc,
            MapPermission::R | MapPermission::W | MapPermission::U,
        ), None);

        let entry = elf_header_part2.entry_point() as usize;

        let context = Context::init_context(
            mode, 
            entry, 
            set.satp_bits(),
            user_stack_top,
            crate::mm::kernel_satp(),
            0,
            crate::trap::trap_handler as usize
        );

        let context_bytes = unsafe {
            core::slice::from_raw_parts(
                &context as *const _ as *const u8, 
                core::mem::size_of::<Context>()
            )
        };

        set.push(MapArea::new(
            SUB_CONTEXT.into(),
            CONTEXT.into(),
            MapType::Alloc,
            MapPermission::R | MapPermission::W,
        ), Some(context_bytes));

        set.push(MapArea::new(
            CONTEXT.into(),
            usize::MAX.into(),
            MapType::Alloc,
            MapPermission::R | MapPermission::W,
        ), Some(context_bytes));

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
            table.map(vpn, frame.ppn(), flags);
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

impl Drop for MemorySet {
    fn drop(&mut self) {
        let table = &mut self.table;
        self.areas.iter().for_each(|area| {
            area.allocated.keys().for_each(|&vpn| {
                table.unmap(vpn);
            })
        })
    }
}

impl Deref for MemorySet {
    type Target = PageTable;

    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

impl Clone for MemorySet {
    fn clone(&self) -> Self {
        let mut set = MemorySet::new();

        self.areas.iter().for_each(|area| {
            let new_area = area.clone();
            set.push(new_area, None);

            if let MapType::Alloc = area.mtype {
                for vpn in area.range {
                    let src_ppn = self.translate(vpn).unwrap().ppn();
                    let dst_ppn = set.translate(vpn).unwrap().ppn();
                    dst_ppn.get_page_bytes().copy_from_slice(
                        src_ppn.get_page_bytes()
                    );
                }
            }
        });

        set
    }
}

impl Clone for MapArea {
    fn clone(&self) -> Self {
        Self {
            range: VPNRange::new(
                self.range.get_start(), 
                self.range.get_end()
            ),
            allocated: BTreeMap::new(),
            ..*self
        }
    }
}
