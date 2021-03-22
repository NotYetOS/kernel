# make is good，ninja??？
MODE := debug
TARGET := riscv64gc-unknown-none-elf
KERNEL_ELF := target/$(TARGET)/$(MODE)/kernel
KERNEL_BIN := kernel.bin
KERNEL_ENTRY := 0x80200000
BOOTLOADER = rustsbi-qemu.bin
FS_IMG = ../fefs-tool/fs.img

build: 
ifeq ($(MODE), debug)
	@cargo build
else
	@cargo build --release
endif
	@@cd ../fefs-tool && cargo run --release

to_bin: build
	@llvm-objcopy $(KERNEL_ELF) $(KERNEL_BIN)

run: to_bin
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY) \
		-drive file=$(FS_IMG),if=none,format=raw,id=x0 \
        -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0

debug: to_bin
	@tmux new-session -d \
		"qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY) \
		-drive file=$(FS_IMG),if=none,format=raw,id=x0 \
        -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 -s -S" && \
		tmux split-window -h "riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && \
		tmux -2 attach-session -d
