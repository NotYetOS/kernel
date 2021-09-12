use crate::mm::{
    frame_alloc, frame_dealloc, kernel_satp, FrameTracker, PageTable, PhysAddr, PhysPageNum,
    VirtAddr,
};
use alloc::vec::Vec;
use fefs::{device::BlockDevice, BLOCK_SIZE};
use lazy_static::lazy_static;
use spin::Mutex;
use virtio_drivers::{VirtIOBlk, VirtIOHeader};

const VIRTIO_MMIO: usize = 0x10001000;

pub struct VirtIOBlock(Mutex<VirtIOBlk<'static>>);

impl VirtIOBlock {
    pub fn new() -> Self {
        Self(Mutex::new(
            VirtIOBlk::new(unsafe { &mut *(VIRTIO_MMIO as *mut VirtIOHeader) }).unwrap(),
        ))
    }
}

impl BlockDevice for VirtIOBlock {
    fn read(&self, addr: usize, buf: &mut [u8]) {
        let block_id = addr / BLOCK_SIZE;
        self.0
            .lock()
            .read_block(block_id, buf)
            .expect("Error when reading VirtIOBlk");
    }

    fn write(&self, addr: usize, buf: &[u8]) {
        let block_id = addr / BLOCK_SIZE;
        self.0
            .lock()
            .write_block(block_id, buf)
            .expect("Error when writing VirtIOBlk");
    }
}

lazy_static! {
    static ref QUEUE_FRAMES: Mutex<Vec<FrameTracker>> = Mutex::new(Vec::new());
}

#[no_mangle]
pub extern "C" fn virtio_dma_alloc(pages: usize) -> PhysAddr {
    let frame = frame_alloc();
    let ppn_base: PhysPageNum = frame.ppn();
    QUEUE_FRAMES.lock().push(frame);

    for _ in 1..pages {
        let frame = frame_alloc();
        QUEUE_FRAMES.lock().push(frame);
    }
    ppn_base.into()
}

#[no_mangle]
pub extern "C" fn virtio_dma_dealloc(pa: PhysAddr, pages: usize) -> i32 {
    let mut ppn_base: PhysPageNum = pa.into();
    for _ in 0..pages {
        frame_dealloc(ppn_base);
        ppn_base += 1;
    }
    0
}

#[no_mangle]
pub extern "C" fn virtio_phys_to_virt(paddr: PhysAddr) -> VirtAddr {
    paddr.value().into()
}

#[no_mangle]
pub extern "C" fn virtio_virt_to_phys(vaddr: VirtAddr) -> PhysAddr {
    match PageTable::from_satp(kernel_satp()).translate_va_to_pa(vaddr) {
        Some(pa) => pa,
        None => panic!("It wasn't supposed to happen"),
    }
}
