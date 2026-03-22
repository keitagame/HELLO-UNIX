# Makefile for ascii-kernel

TARGET      := x86_64-unknown-none
KERNEL_ELF  := kernel.elf
ISO         := ascii-kernel.iso

# Tools
NASM        := nasm
LD          := ld
RUSTC       := rustc

# Rust compile flags
RUSTFLAGS   := --edition 2021 \
               --crate-type staticlib \
               --crate-name kernel \
               -C opt-level=2 \
               -C panic=abort \
               --target $(TARGET) \
               -C code-model=kernel \
               -C relocation-model=static

.PHONY: all clean iso

all: $(ISO)

# 1. Compile Rust → libkernel.a
libkernel.a: src/main.rs
	$(RUSTC) $(RUSTFLAGS) src/main.rs -o $@

# 2. Assemble boot stub
boot.o: src/boot.asm
	$(NASM) -f elf32 src/boot.asm -o $@

# 3. Link everything
$(KERNEL_ELF): boot.o libkernel.a linker.ld
	$(LD) -m elf_x86_64 -T linker.ld -o $@ boot.o -L. -lkernel --no-pie -z noexecstack

# 4. Build ISO
$(ISO): $(KERNEL_ELF)
	cp $(KERNEL_ELF) isodir/boot/kernel.elf
	grub-mkrescue -o $@ isodir

clean:
	rm -f boot.o libkernel.a $(KERNEL_ELF) $(ISO) isodir/boot/kernel.elf
