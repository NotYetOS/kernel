.altmacro
.macro LOAD n 
    ld x\n, \n*8(sp)
.endm

    .section .text.load
    .global _load
    .align 4

_load: 
    mv t0, sp
    li sp, 0xffffffffffffe000
    csrw satp, a0
    sfence.vma

    sd t0, 36*8(sp)

    ld t0, 2*8(sp)
    ld t1, 32*8(sp)
    ld t2, 33*8(sp)
    ld t3, 34*8(sp)

    csrw sstatus, t1
    csrw sepc, t2
    csrw satp, t3

    li x1, 0
    li x3, 0

    .set n, 6
    .rept 26
        LOAD %n
        .set n, n + 1
    .endr

    mv sp, t0 

    sret
    