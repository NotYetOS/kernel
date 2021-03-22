.altmacro
.macro STORE n
    sd x\n, \n*8(sp)
.endm

.macro LOAD n 
    ld x\n, \n*8(sp)
.endm

    .section .text.trap
    .global _entry
    .global _restore
    
    # stvec need align 4 byte
    .align 4
_entry: 
    csrrw sp, sscratch, sp

    # alloc space
    addi sp, sp, -34*8

    sd x1, 1*8(sp)
    sd x3, 3*8(sp)

    .set n, 4
    .rept 28
        STORE %n
        .set n, n + 1
    .endr

    csrr t0, sscratch
    csrr t1, sstatus
    csrr t2, sepc

    sd t0, 2*8(sp)
    sd t1, 32*8(sp)
    sd t2, 33*8(sp)

    # equel addi a0, sp, 0
    # input function argument
    mv a0, sp
    call trap_handler

_restore: 
    # get return value
    mv sp, a0

    ld x1, 1*8(sp)
    ld x3, 3*8(sp)

    ld t0, 2*8(sp)
    ld t1, 32*8(sp)
    ld t2, 33*8(sp)

    csrw sscratch, t0
    csrw sstatus, t1
    csrw sepc, t2

    .set n, 4
    .rept 28
        LOAD %n
        .set n, n + 1
    .endr

    # dealloc space 
    addi sp, sp, 34*8
    csrrw sp, sscratch, sp

    // return with apply status
    sret