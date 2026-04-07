use crate::utilities::{clear_reg_bits, read_reg, set_reg_bits, write_reg};

struct UartPl011 {
    base_addr: *mut u32,
    base_clock: u32,
    baudrate: u32,
    data_bits: u8,
    stop_bits: u8,
}

// Register Offsets (in 32-bit words, not bytes, matching your logic)
const DR_OFF: usize = 0x00;
const FR_OFF: usize = 0x18; // 0x6
const IBRD_OFF: usize = 0x24; // 0x9
const FBRD_OFF: usize = 0x28; // 0xA
const LCR_OFF: usize = 0x2c; // 0xB
const CR_OFF: usize = 0x30; // 0xC
const IMSC_OFF: usize = 0x38; // 0xE
const DMACR_OFF: usize = 0x48; // 0x12

const FR_BUSY: u32 = 1 << 3;
const LCR_FEN: u32 = 1 << 4;
const LCR_STP2: u32 = 1 << 3;
const CR_UARTEN: u32 = 1 << 0;
const CR_TXEN: u32 = 1 << 8;

static mut UART: UartPl011 = UartPl011 {
    base_addr: core::ptr::null_mut(),
    base_clock: 0,
    baudrate: 0,
    data_bits: 0,
    stop_bits: 0,
};

#[unsafe(no_mangle)]
pub extern "C" fn init_uart(base_addr: *mut u32, base_clock: u32, baudrate: u32) {
    unsafe {
        UART = UartPl011 {
            base_addr: base_addr,
            base_clock: base_clock,
            baudrate: baudrate,
            data_bits: 8,
            stop_bits: 1,
        };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn configure_uart() {
    // PL011 Sequence

    // 1. Disable UART (Clear UARTEN)
    // We strictly write 0 directly to Control Register
    unsafe {
        clear_reg_bits(UART.base_addr as usize, CR_OFF, CR_UARTEN);
    }

    // 2. Wait for current TX to finish
    // On QEMU this is instant, on hardware loop until !BUSY
    unsafe { while (read_reg(UART.base_addr as usize, FR_OFF) & FR_BUSY) != 0 {} }

    // 3. Flush FIFO (Enable FEN)
    unsafe {
        set_reg_bits(UART.base_addr as usize, LCR_OFF, LCR_FEN);
    }

    // 4. Set Baud Rate
    // Since QEMU ignores this usually, we keep your logic but ensure no overflow
    unsafe {
        // Cast to u64 to prevent overflow: 24MHz * 1000 > u32 max
        let base_clock = UART.base_clock as u64;
        let baud = UART.baudrate as u64;
        // Avoid divide by zero
        if baud > 0 {
            let baud_div = (base_clock * 1000) / (16 * baud);
            let ibrd = (baud_div / 1000) as u32;
            let fbrd = (((baud_div % 1000) * 64 + 500) / 1000) as u32;

            write_reg(UART.base_addr as usize, IBRD_OFF, ibrd);
            write_reg(UART.base_addr as usize, FBRD_OFF, fbrd & 0x3f);
        }
    }

    // 5. Line Control (8 bits, 1 stop bit, FIFO enabled)

    let mut lcr: u32 = 0;

    unsafe {
        // Calculate WLEN (Word Length)
        // (8 - 1) & 3 -> 7 & 3 -> 3 (b11). Shift left 5 -> b1100000 (Bits 6,5)
        lcr |= ((UART.data_bits - 1) as u32 & 0x3) << 5;

        // Check for 2 Stop Bits
        if UART.stop_bits == 2 {
            lcr |= LCR_STP2;
        }
    }

    // Always enable FIFO (Bit 4)
    lcr |= LCR_FEN;

    // Write the calculated configuration to LCR_H
    unsafe {
        write_reg(UART.base_addr as usize, LCR_OFF, lcr);
    }

    // 6. Mask Interrupts (Write 0 to IMSC)
    unsafe {
        write_reg(UART.base_addr as usize, IMSC_OFF, 0);
    }

    unsafe {
        clear_reg_bits(UART.base_addr as usize, DMACR_OFF, 0x7);
    }

    // 7. Enable UART (UARTEN + TXE)
    unsafe {
        write_reg(UART.base_addr as usize, CR_OFF, CR_UARTEN | CR_TXEN);
    }
}

fn putchar(c: u8) {
    unsafe {
        while (read_reg(UART.base_addr as usize, FR_OFF) & FR_BUSY) != 0 {}
        let addr = UART.base_addr as *mut u8;
        core::ptr::write_volatile(addr.add(DR_OFF), c);
    }
}

pub fn print(s: &[u8]) {
    for &c in s {
        putchar(c);
    }
}
