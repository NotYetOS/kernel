OUTPUT_ARCH(riscv)
ENTRY(_start)
START_ADDRESS = 0x80200000;

SECTIONS
{
    . = START_ADDRESS;

    .text : {
        *(.text.entry)
        . = ALIGN(4K);
        *(.text .text.*)
    }

    . = ALIGN(4K);

    .rodata : {
        *(.rodata .rodata.*)
    }

    . = ALIGN(4K);

    .data : {
        *(.data .data.*)
    }
    
    . = ALIGN(4K);

    .bss : {
        *(.bss.stack)
        *(.bss .bss.*)
    }

    . = ALIGN(4K);
}
