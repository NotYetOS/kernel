.section .text.asm
.global _trap_entry
.global _restore
.global _restore_hart

# stvec need align 4 byte
.align 4

_trap_entry: 
    # temp store sp
    csrw sscratch, sp

    # set context addr
    li sp, 0xfffffffffffff000

    sd ra, 1*8(sp)
    sd gp, 3*8(sp)
    sd tp, 4*8(sp)
    sd t0, 5*8(sp)
    sd t1, 6*8(sp)
    sd t2, 7*8(sp)
    sd s0, 8*8(sp)
    sd s1, 9*8(sp)
    sd a0, 10*8(sp)
    sd a1, 11*8(sp)
    sd a2, 12*8(sp)
    sd a3, 13*8(sp)
    sd a4, 14*8(sp)
    sd a5, 15*8(sp)
    sd a6, 16*8(sp)
    sd a7, 17*8(sp)
    sd s2, 18*8(sp)
    sd s3, 19*8(sp)
    sd s4, 20*8(sp)
    sd s5, 21*8(sp)
    sd s6, 22*8(sp)
    sd s7, 23*8(sp)
    sd s8, 24*8(sp)
    sd s9, 25*8(sp)
    sd s10, 26*8(sp)
    sd s11, 27*8(sp)
    sd t3, 28*8(sp)
    sd t4, 29*8(sp)
    sd t5, 30*8(sp)
    sd t6, 31*8(sp)

    csrr t1, sscratch
    sd t1, 2*8(sp)
    
    csrr t1, sepc
    sd t1, 33*8(sp)
    
    csrr t1, satp
    sd t1, 34*8(sp)

    ld t1, 38*8(sp)

    # judge from user or kernel
    beq t1, x0, _from_user; 
    bne t1, x0, _from_kernel;
    
_from_user:
    # store sstatus
    csrr t1, sstatus
    sd t1, 32*8(sp)

    # user satp
    ld t1, 34*8(sp)

    # kernel satp
    ld t2, 35*8(sp)

    # kernel sp
    ld t3, 36*8(sp)

    # trap_handler
    ld t4, 37*8(sp)

    # switch to kernel space
    csrw satp, t2
    sfence.vma

    # store user satp to kernel space
    sd t1, 34*8(sp)

    # switch to kernel sp
    mv sp, t3

    # jump to trap_handler
    jr t4

_from_kernel:
    # no need to store sstatus

    # trap_handler
    ld t1, 37*8(sp)

    # restore kernel sp
    csrr sp, sscratch

    # jump to trap_handler
    jr t1

_restore:
    # set context addr
    li sp, 0xfffffffffffff000

    # load target satp
    ld t1, 34*8(sp)

    # switch to target space
    csrw satp, t1
    sfence.vma

    ld t1, 32*8(sp)
    ld t2, 33*8(sp)
    csrw sstatus, t1
    csrw sepc, t2

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

_restore_hart:
    # set context addr
    mv sp, a1

    ld t1, 32*8(sp)
    ld t2, 33*8(sp)
    csrw sstatus, t1
    csrw sepc, t2

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
