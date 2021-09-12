pub const PAGE_SIZE: usize = 4096;
pub const PAGE_SIZE_BITS: usize = 12;
pub const KERNEL_HEAP_SIZE: usize = 0x30_0000;
pub const MEMORY_END: usize = 0x80800000;
pub const USER_STACK_SIZE: usize = 4096 * 2;
pub const CONTEXT: usize = usize::MAX - PAGE_SIZE + 1;
pub const CLOCK_FREQ: usize = 12500000;

pub const MMIO: &[(usize, usize)] = &[(0x10001000, 0x1000)];
