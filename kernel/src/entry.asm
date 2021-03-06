    .section .text.entry
    .global _start
_start: 
    # load stack_top to sp register
    la sp, stack_top
    call main

    .section .bss.stack
    .global stack
stack:
    # alloc stack memory
    .space 4096 * 16
    .global stack_top
stack_top: