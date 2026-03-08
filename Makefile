TARGET := aarch64-unknown-none
AS := aarch64-elf-as
ASFLAGS := -I./include
CC := aarch64-elf-gcc
CFLAGS := -Wall -ggdb -ffreestanding -nostdlib -I./include
LLVM_TOOLS_PATH := $(shell rustc --print sysroot)/lib/rustlib/$(shell rustc -vV | grep host | cut -d ' ' -f 2)/bin
LD := $(LLVM_TOOLS_PATH)/rust-lld
QEMU := qemu-system-aarch64
VERSION := debug

SRC_DIR := src
ASM_DIR := $(SRC_DIR)/asm
RUST_SRC := $(shell find $(SRC_DIR) -name "*.rs")
ASM_SRC := $(shell find $(SRC_DIR) -name "*.s" )
LINKER_SCRIPT := linker.ld

ASM_OBJS := $(ASM_SRC:.s=.o)
CRATE_NAME := $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].name')
KERNEL_OBJ := target/$(TARGET)/$(VERSION)/lib$(CRATE_NAME).a
KERNEL_ELF := kernel.elf

QEMU_FLAGS := -machine virt -cpu cortex-a57 -no-graphic -kernel $(KERNEL_ELF)

all: $(KERNEL_ELF)

$(ASM_DIR)/%.o : $(ASM_DIR)/%.s
	$(AS) $(ASFLAGS) -o $@ $<

$(KERNEL_OBJ): $(KERNEL_RS)
	RUSTFLAGS="-C target-feature=-neon" cargo build --target $(TARGET)

$(KERNEL_ELF): $(ASM_OBJS) $(KERNEL_OBJ) $(LINKER_SCRIPT)
	$(LD) -flavor gnu -o $(KERNEL_ELF) -T $(LINKER_SCRIPT) $(ASM_OBJS) $(KERNEL_OBJ)

run: $(KERNEL_ELF)
	$(QEMU) $(QEMU_FLAGS)

clean:
	cargo clean
	rm -rf target
	rm -f $(ASM_OBJS) $(KERNEL_ELF)

.PHONY: all run clean
