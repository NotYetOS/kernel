OUTPUT_ARCH(riscv)

# 入口
ENTRY(_start)

# 为什么是0x80200000？
# 因为rust-sbi初始化完后进入的地址为0x80200000
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    # 代码段
    .text : {
        # 链接入口
        *(.text.entry)

        # 4K对齐
        . = ALIGN(4K);

        # 匹配所有段
        *(.text .text.*)
    }

    # 4K对齐
    . = ALIGN(4K);

    # 只读数据段
    .rodata : {
        # 匹配所有段
        *(.rodata .rodata.*)
    }

    # 4K对齐
    . = ALIGN(4K);

    # 数据段
    .data : {
        # 匹配所有段
        *(.data .data.*)
    }
    
    # 4K对齐
    . = ALIGN(4K);

    # 我也不知道这个段该怎么说
    .bss : {
        # 匹配所有段
        *(.bss .bss.*)
    }

    # 4K对齐
    . = ALIGN(4K);
}