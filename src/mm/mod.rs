// impl Sv39
mod address;
mod allocators;
mod pte;
mod set;
mod table;

use alloc::sync::Arc;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref KERNEL_SPACE: Arc<Mutex<MemorySet>> =
        Arc::new(Mutex::new(MemorySet::new_kernel()));
}

// map sections and activate Sv39
pub fn init() {
    init_heap();
    KERNEL_SPACE.lock().activate(Mode::Sv39);
}

pub fn kernel_satp() -> usize {
    KERNEL_SPACE.lock().satp_bits()
}

// to export
pub use address::*;
pub use allocators::*;
pub use pte::*;
pub use set::*;
pub use table::*;
