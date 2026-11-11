# Stark OS (`stark-os`)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-nightly--aarch64-orange.svg)](https://www.rust-lang.org/)
[![Target: ARMv8-A / QEMU virt](https://img.shields.io/badge/arch-AArch64%20%7C%20ARMv8--A-blue.svg)]()

`stark-os` is a bare-metal operating system kernel built from scratch for **AArch64 (ARMv8-A)** using **Rust** (`#![no_std]`) and native **ARM64 Assembly**. 

Designed to target the `qemu-system-aarch64` `virt` machine architecture, `stark-os` features custom lower-level drivers for PL011 UART communication, ARM Generic Interrupt Controller (GIC) initialization, vector exception handling, and custom bootloader vector table orchestration.

---

## 🏛️ Architecture & Component Layout

```
stark-os/
├── Cargo.toml         # Rust manifest configured for freestanding AArch64 target
├── Makefile           # Build system for assembling, linking, and QEMU execution
├── linker.ld          # Custom linker script mapping vector table & section addresses
├── include/
│   └── macros.inc     # Assembly macros for register saving/restoring
└── src/
    ├── asm/
    │   ├── boot.s     # Low-level EL1/EL2 entry point & stack setup
    │   └── interrupt.s# Exception Vector Table (EVT) & trap handlers
    ├── lib.rs         # Kernel entry point (`kernel_main`), panic handler, Rust initialization
    ├── uart.rs        # PL011 UART MMIO driver & formatting macros (`print!`, `println!`)
    ├── gic.rs         # ARM Generic Interrupt Controller (GICv2/v3) register setup
    ├── expections.rs  # High-level Rust exception dispatchers & registers logging
    └── utilities.rs   # Low-level system register operations & bitwise helpers
```

---

## ⚡ Core Technical Features

### 1. Low-Level Bootstrapping & ASM Assembly
- **Bare-Metal Entry Point (`src/asm/boot.s`)**: Hands off control from CPU initialization directly to `_start`, setting up execution modes, initial stacks, zeroing the `.bss` memory region, and jumping to Rust's `kernel_main`.
- **Exception Vector Table (`src/asm/interrupt.s`)**: Installs an ARM64-compliant 2,048-byte aligned Exception Vector Table targeting `VBAR_EL1`.
- **Macro Assembly Helpers (`include/macros.inc`)**: Provides fast push/pop routines for context preservation across exception handler boundaries (saving/restoring registers `x0`-`x30` and system state registers).

### 2. ARM Subsystem & Driver Implementations
- **PL011 UART Driver (`src/uart.rs`)**: Memory-mapped I/O (MMIO) driver providing serial line communication, integrated into Rust's `core::fmt::Write` interface for macro support (`print!`, `println!`).
- **Generic Interrupt Controller (`src/gic.rs`)**: Manages GIC Distributor (`GICD`) and CPU Interface (`GICC`) MMIO regions to route, enable, and handle hardware IRQs and software interrupts.
- **Exception & Trap Management (`src/expections.rs`)**: Bridges assembly context frames into safe Rust structures, logging hardware exceptions, synchronous traps, IRQs, and system fault states with register dumps.
- **Register Utilities (`src/utilities.rs`)**: Inline assembly wrappers for accessing ARM control registers (`DAIF`, `ESR_EL1`, `FAR_EL1`, `SPSR_EL1`, `ELR_EL1`).

---

## 🛠️ Toolchain & Prerequisites

To assemble, cross-compile, and emulate `stark-os`, install the following prerequisites:

- **Rust Nightly Toolchain** with `aarch64-unknown-none` target:
  ```bash
  rustup target add aarch64-unknown-none
  ```
- **GCC ARM64 Cross-Assembler & Linker**:
  ```bash
  # macOS
  brew install aarch64-elf-gcc

  # Ubuntu / Debian
  sudo apt-get install gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu
  ```
- **QEMU System Emulator (`qemu-system-aarch64`)**:
  ```bash
  # macOS
  brew install qemu

  # Ubuntu / Debian
  sudo apt-get install qemu-system-arm
  ```

---

## 🚀 Building & Execution

The project relies on a `Makefile` to orchestrate assembling `.s` assembly files, compiling the Rust `#![no_std]` binary crate, and running ELF targets.

### 1. Build the OS Kernel
To assemble boot routines and compile the kernel:
```bash
make all
```

### 2. Run inside QEMU Emulate (ARM64 Virt Board)
Boot the compiled OS inside `qemu-system-aarch64` using the PL011 UART terminal:
```bash
make run
```

### 3. Clean Build Artifacts
```bash
make clean
```

---

## 🔍 Low-Level GDB Remote Debugging

To debug boot vector initialization, register transitions, or exception handling:

1. Launch QEMU with a GDB server socket enabled (`-s -S` flags):
   ```bash
   qemu-system-aarch64 -M virt -cpu cortex-a53 -nographic -kernel build/stark-os.elf -s -S
   ```

2. Connect `gdb-multiarch` or `lldb` from a separate shell:
   ```bash
   gdb-multiarch build/stark-os.elf
   (gdb) target remote localhost:1234
   (gdb) break _start
   (gdb) break kernel_main
   (gdb) continue
   ```

---

## 🎯 Development Roadmap

- [x] ARM64 `_start` boot sequence & `.bss` clearing
- [x] Memory-Mapped PL011 UART output driver
- [x] ARM64 Exception Vector Table (`VBAR_EL1`) setup
- [x] Generic Interrupt Controller (GICv3)
- [ ] MMU Translation Tables & Virtual Memory Paging (`TBR0_EL1` / `TBR1_EL1`)
- [ ] Dynamic Heap Allocation (`alloc` crate implementation)
- [ ] Multitasking & Cooperative Context Switching Engine

---

## 📜 License

Distributed under the MIT License. See `LICENSE` for details.
