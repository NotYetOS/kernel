use riscv::register::sstatus::{
    self, 
    SPP, 
    Sstatus,
};

pub struct TrapContext {
    pub x: [usize; 32],
    // save status, like privilege mode
    pub sstatus: Sstatus,
    pub sepc: usize
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) { self.x[2] = sp; }
    pub fn app_trap_context(mode: SPP, entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(mode);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
        };
        cx.set_sp(sp);
        cx
    }
}
