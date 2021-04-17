use riscv::register::sstatus::SPP;
use crate::config::*;
use crate::mm::PageTable;

#[repr(C)]
struct CallContext {
    zero: usize,
    ra: usize,
    s0: usize,
    s1: usize,
    s2: usize,
    s3: usize,
    s4: usize,
    s5: usize,
    s6: usize,
    s7: usize,
    s8: usize,
    s9: usize,
    s10: usize,
    s11: usize,
    sp: usize,
}

impl Default for CallContext {
    fn default() -> Self {
        Self {
            zero: 0,
            ra: 0,
            s0: 0,
            s1: 0,
            s2: 0,
            s3: 0,
            s4: 0,
            s5: 0,
            s6: 0,
            s7: 0,
            s8: 0,
            s9: 0,
            s10: 0,
            s11: 0,
            sp: 0,
        }
    }
}

#[repr(C)]
pub struct Context {
    pub zero: usize,
    pub ra: usize,
    pub sp: usize,
    pub gp: usize,
    pub tp: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub s0: usize,
    pub s1: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
    // save status, like privilege mode
    pub sstatus: usize,
    pub sepc: usize,
    pub satp: usize,
    kernel_satp: usize,
    kernel_sp: usize,
    trap_handler: usize,
    spp: usize,
    is_in_call_process: usize,
    call_context: CallContext
}

impl Context {
    pub fn init_context(
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

        Self {
            zero: 0,
            ra: 0,
            sp,
            gp: 0,
            tp: 0,
            t0: 0,
            t1: 0,
            t2: 0,
            s0: 0,
            s1: 0,
            a0: 0,
            a1: 0,
            a2: 0,
            a3: 0,
            a4: 0,
            a5: 0,
            a6: 0,
            a7: 0,
            s2: 0,
            s3: 0,
            s4: 0,
            s5: 0,
            s6: 0,
            s7: 0,
            s8: 0,
            s9: 0,
            s10: 0,
            s11: 0,
            t3: 0,
            t4: 0,
            t5: 0,
            t6: 0,
            sstatus,
            sepc: entry,
            satp,
            kernel_satp,
            kernel_sp,
            trap_handler,
            spp: mode as usize,
            is_in_call_process: 0,
            call_context: CallContext::default()
        }
    }
}

pub fn get_context(satp: usize) -> &'static mut Context {
    let pt = PageTable::from_satp(satp);
    let pa = pt.translate_va_to_pa(CONTEXT.into()).unwrap();
    pa.get_mut::<Context>()
}
