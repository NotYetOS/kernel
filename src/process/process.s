.altmacro
.macro STORE n
    sd x\n, \n*8(sp)
.endm

.macro LOAD n 
    ld x\n, \n*8(sp)
.endm

.section .text.asm
.global _load
.global _save_call_context
.global _ret
.align 4

_load: 
    csrw sscratch, sp
    li sp, 0xffffffffffffe000

    sd x1, 1*8(sp)
    .set n, 3
    .rept 29
        STORE %n
        .set n, n + 1
    .endr

    csrr t0, sscratch
    csrr t1, sepc
    csrr t2, satp

    sd t0, 2*8(sp)
    sd t1, 33*8(sp)
    sd t2, 34*8(sp)

    csrw satp, a0
    sfence.vma

    li sp, 0xfffffffffffff000
    ld t1, 39*8(sp)
    beq t1, x0, _back_to_user; 
    bne t1, x0, _back_to_call_process;
    
_back_to_user:
    li sp, 0xfffffffffffff000

    sd t0, 36*8(sp)
    ld t0, 2*8(sp)
    ld t1, 32*8(sp)
    ld t2, 33*8(sp)
    ld t3, 34*8(sp)

    csrw sstatus, t1
    csrw sepc, t2
    csrw satp, t3

    ld x1, 1*8(sp)
    ld x3, 3*8(sp)

    .set n, 6
    .rept 26
        LOAD %n
        .set n, n + 1
    .endr

    mv sp, t0
    li t0, 0
    sret

_back_to_call_process: 
    li t1, 0
    sd t1, 39*8(sp)

    li sp, 0xffffffffffffe000

    ld t0, 2*8(sp)
    ld t1, 33*8(sp)
    ld t2, 34*8(sp)

    csrw sepc, t1
    csrw satp, t2

    ld x1, 1*8(sp)
    ld x3, 3*8(sp)

    .set n, 6
    .rept 26
        LOAD %n
        .set n, n + 1
    .endr

    mv sp, t0
    li t0, 0
    sret

_save_call_context:
    csrw sscratch, sp
    csrw satp, a0
    sfence.vma

    li sp, 0xffffffffffffe000

    sd x1, 1*8(sp)
    .set n, 3
    .rept 29
        STORE %n
        .set n, n + 1
    .endr

    csrr t0, sscratch
    csrr t1, sepc
    csrr t2, satp

    sd t0, 2*8(sp)
    sd t1, 33*8(sp)
    sd t2, 34*8(sp)

    ld t0, 35*8(sp)

    csrw satp, t0
    sfence.vma

    ret
    
_ret: 
    li sp, 0xffffffffffffe000

    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    csrw sstatus, t0
    csrw sepc, t1

    ld x1, 1*8(sp)
    ld x3, 3*8(sp)

    .set n, 5
    .rept 27
        LOAD %n
        .set n, n + 1
    .endr

    ld sp, 2*8(sp)
    ret
    