# make is good，ninja??？
MODE := debug
TARGET := riscv64gc-unknown-none-elf
KERNEL_ELF := target/$(TARGET)/$(MODE)/kernel
KERNEL_BIN := kernel.bin
KERNEL_ENTRY := 0x80200000
BOOTLOADER = fw_payload.bin

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

debug: to_bin
	@tmux new-session -d \
		"qemu-system-riscv64 -machine virt -nographic -bios $(BOOTLOADER) -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY) -s -S" && \
		tmux split-window -h "riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && \
		tmux -2 attach-session -d
