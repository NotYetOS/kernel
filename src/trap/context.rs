use riscv::register::sstatus::{
    self, 
    SPP, 
    Sstatus,
};

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    // save status, like privilege mode
    pub sstatus: Sstatus,
    pub sepc: usize,
    pub satp: usize,
    pub kernel_satp: usize,
    pub kernel_sp: usize,
    pub trap_handler: usize,
    spp: usize
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) { self.x[2] = sp; }
    pub fn init_trap_context(
        mode: SPP, 
        entry: usize, 
        satp: usize, 
        sp: usize,
        kernel_satp: usize,
        kernel_sp: usize,
        trap_handler: usize
    ) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(mode);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
            satp,
            kernel_satp,
            kernel_sp,
            trap_handler,
            spp: mode as usize,
        };
        cx.set_sp(sp);
        cx
    }
}
