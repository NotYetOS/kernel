OUTPUT_ARCH(riscv)
ENTRY(_start)
START_ADDRESS = 0x80200000;

SECTIONS
{
    . = START_ADDRESS;

    skernel = .;

    stext = .;
    .text : {
        *(.text.entry)
        . = ALIGN(4K);

        strampoline = .;
        *(.text.trampoline);
        . = ALIGN(4K);
        etrampoline = .;

        *(.text .text.*)
    }
    etext = .;

    . = ALIGN(4K);

    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
    }
    erodata = .;

    . = ALIGN(4K);

    sdata = .;
    .data : {
        *(.data .data.*)
    }
    edata = .;
    
    . = ALIGN(4K);

    sbss = .;
    sstack = .;
    .bss : {
        *(.bss.stack)
        estack = .;
        *(.bss .bss.*)
    }
    ebss = .;

    . = ALIGN(4K);
    ekernel = .;
}
