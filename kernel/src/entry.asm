    .section .text.entry
    .global _start
_start: 
    la sp, stack_top
    call main

    .section .bss.stack
    .global stack
stack:
    .space 4096 * 16
    .global stack_top
stack_top: