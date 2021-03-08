// impl Sv39
mod address;
mod pte;
mod table;
mod allocators;
mod set;

// mapping sections and activate Sv39
pub fn init() {
    init_heap();
    let kernel = MemorySet::new_kernel();
    kernel.activate(Mode::Sv39);
}

// to export
pub use address::*;
pub use pte::*;
pub use allocators::*;
pub use table::*;
pub use set::*;
