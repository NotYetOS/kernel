.section .text.asm
.global _load
.global _save_call_context
.global _ret
.align 4

_load: 
    csrw sscratch, sp
    # call context addr
    li sp, 0xfffffffffffff160

    sd ra, 1*8(sp)
    sd s0, 2*8(sp)
    sd s1, 3*8(sp)
    sd s2, 4*8(sp)
    sd s3, 5*8(sp)
    sd s4, 6*8(sp)
    sd s5, 7*8(sp)
    sd s6, 8*8(sp)
    sd s7, 9*8(sp)
    sd s8, 10*8(sp)
    sd s9, 11*8(sp)
    sd s10, 12*8(sp)
    sd s11, 13*8(sp)

    # store t1
    sd t1, 0*8(sp)

    # store kernel sp
    csrr t1, sscratch
    sd t1, 14*8(sp)

    # restore t1
    ld t1, 0*8(sp)
    sd x0, 0*8(sp)

    csrw satp, a0
    sfence.vma

    li sp, 0xfffffffffffff000
    ld t1, 39*8(sp)
    beq t1, x0, _back_to_user; 
    bne t1, x0, _back_to_call_process;
    
_back_to_user:
    # store t1
    sd t1, 0*8(sp)

    # store kernel sp
    csrr t1, sscratch
    sd t1, 36*8(sp)

    # load sstatus
    ld t1, 32*8(sp)
    csrw sstatus, t1

    # load sepc
    ld t1, 33*8(sp)
    csrw sepc, t1

    # restore t1
    ld t1, 0*8(sp)
    sd x0, 0*8(sp)

    ld ra, 1*8(sp)
    ld gp, 3*8(sp)
    ld tp, 4*8(sp)
    ld t0, 5*8(sp)
    ld t1, 6*8(sp)
    ld t2, 7*8(sp)
    ld s0, 8*8(sp)
    ld s1, 9*8(sp)
    ld a0, 10*8(sp)
    ld a1, 11*8(sp)
    ld a2, 12*8(sp)
    ld a3, 13*8(sp)
    ld a4, 14*8(sp)
    ld a5, 15*8(sp)
    ld a6, 16*8(sp)
    ld a7, 17*8(sp)
    ld s2, 18*8(sp)
    ld s3, 19*8(sp)
    ld s4, 20*8(sp)
    ld s5, 21*8(sp)
    ld s6, 22*8(sp)
    ld s7, 23*8(sp)
    ld s8, 24*8(sp)
    ld s9, 25*8(sp)
    ld s10, 26*8(sp)
    ld s11, 27*8(sp)
    ld t3, 28*8(sp)
    ld t4, 29*8(sp)
    ld t5, 30*8(sp)
    ld t6, 31*8(sp)
    ld sp, 2*8(sp)

    sret

_back_to_call_process: 
    # store t1
    sd t1, 0*8(sp)

    # clear flag
    li t1, 0
    sd t1, 39*8(sp)

    # restore t1
    ld t1, 0*8(sp)
    sd x0, 0*8(sp)

    li sp, 0xfffffffffffff160
    ld ra, 1*8(sp)
    ld s0, 2*8(sp)
    ld s1, 3*8(sp)
    ld s2, 4*8(sp)
    ld s3, 5*8(sp)
    ld s4, 6*8(sp)
    ld s5, 7*8(sp)
    ld s6, 8*8(sp)
    ld s7, 9*8(sp)
    ld s8, 10*8(sp)
    ld s9, 11*8(sp)
    ld s10, 12*8(sp)
    ld s11, 13*8(sp)
    ld sp, 14*8(sp)

    ret

_save_call_context:
    csrw sscratch, sp
    li sp, 0xfffffffffffff160

    csrw satp, a0
    sfence.vma

    sd ra, 1*8(sp)
    sd s0, 2*8(sp)
    sd s1, 3*8(sp)
    sd s2, 4*8(sp)
    sd s3, 5*8(sp)
    sd s4, 6*8(sp)
    sd s5, 7*8(sp)
    sd s6, 8*8(sp)
    sd s7, 9*8(sp)
    sd s8, 10*8(sp)
    sd s9, 11*8(sp)
    sd s10, 12*8(sp)
    sd s11, 13*8(sp)

    # store t1
    sd t1, 0*8(sp)

    # store kernel sp
    csrr t1, sscratch
    sd t1, 14*8(sp)

    # restore t1
    ld t1, 0*8(sp)
    sd x0, 0*8(sp)

    csrw satp, a1
    sfence.vma

    csrr sp, sscratch
    ret
    
_ret: 
    li sp, 0xfffffffffffff160

    ld ra, 1*8(sp)
    ld s0, 2*8(sp)
    ld s1, 3*8(sp)
    ld s2, 4*8(sp)
    ld s3, 5*8(sp)
    ld s4, 6*8(sp)
    ld s5, 7*8(sp)
    ld s6, 8*8(sp)
    ld s7, 9*8(sp)
    ld s8, 10*8(sp)
    ld s9, 11*8(sp)
    ld s10, 12*8(sp)
    ld s11, 13*8(sp)
    ld sp, 14*8(sp)

    ret
