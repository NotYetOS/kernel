use riscv::register::sstatus::SPP;

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    // save status, like privilege mode
    sstatus: usize,
    pub sepc: usize,
    satp: usize,
    kernel_satp: usize,
    kernel_sp: usize,
    trap_handler: usize,
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
        let sstatus = match mode {
            SPP::Supervisor => 1 << 8,
            SPP::User => 0
        };

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
