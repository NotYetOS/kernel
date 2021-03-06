    .section .text.entry
    .global _start
_start: 
    # 栈地址写进sp
    la sp, stack_top
    call main

    .section .bss.stack
    .global stack
stack:
    # 分配栈空间
    .space 4096 * 16
    .global stack_top
stack_top: