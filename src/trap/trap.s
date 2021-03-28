.altmacro
.macro STORE n
    sd x\n, \n*8(sp)
.endm

.macro LOAD n 
    ld x\n, \n*8(sp)
.endm

.section .text.trampoline
.global _trap_entry
.global _restore

# stvec need align 4 byte
.align 4

_trap_entry: 
    csrw sscratch, sp
    li sp, 0xffffffffffffe000

    sd x1, 1*8(sp)
    .set n, 3
    .rept 29
        STORE %n
        .set n, n + 1
    .endr

    csrr t0, sscratch
    csrr t1, sstatus
    csrr t2, sepc
    csrr t3, satp

    sd t0, 2*8(sp)
    sd t1, 32*8(sp)
    sd t2, 33*8(sp)
    sd t3, 34*8(sp)

    ld t1, 38*8(sp)
    beq t1, x0, _from_user; 
    bne t1, x0, _from_kernel;
    
_from_user:
    ld t0, 34*8(sp)
    ld t1, 35*8(sp)
    ld t2, 36*8(sp)
    ld t3, 37*8(sp)
    csrw satp, t1
    sfence.vma
    sd t0, 34*8(sp)
    mv sp, t2
    jr t3

_from_kernel:
    ld t0, 35*8(sp)
    ld t2, 37*8(sp)
    csrr sp, sscratch
    csrw satp, t0
    sfence.vma
    jr t2

_restore:
    li sp, 0xffffffffffffe000
    ld t0, 34*8(sp)
    csrw satp, t0
    sfence.vma

    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    csrw sstatus, t0
    csrw sepc, t1

    ld x1, 1*8(sp)
    ld x3, 3*8(sp)

    .set n, 5
    .rept 27
        LOAD %n
        .set n, n+1
    .endr

    ld sp, 2*8(sp)
    sret
    