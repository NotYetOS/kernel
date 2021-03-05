# make大法好，ninja那是人用的？
MODE := debug
TARGET := riscv64gc-unknown-none-elf
KERNEL_ELF := target/$(TARGET)/$(MODE)/kernel
KERNEL_BIN := kernel.bin
KERNEL_ENTRY := 0x80200000
BOOTLOADER = rustsbi-qemu.bin

build: 
ifeq ($(MODE), debug)
	@cargo build
else
	@cargo build --release
endif

to_bin: build
	@llvm-objcopy $(KERNEL_ELF) $(KERNEL_BIN)

run: to_bin
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY)