struct UartPL011 {
    base_addr: *mut u32,
    base_clock: u32,
    baudrate: u32,
    data_bits: u8,
    stop_bits: u8,
}

const DR_OFF: isize = 0x00;
const FR_OFF: isize = 0x18 / 4;
const IBRD_OFF: isize = 0x24 / 4;
const FBRD_OFF: isize = 0x28 / 4;
const LCR_OFF: isize = 0x2c / 4;
const CR_OFF: isize = 0x30 / 4;
const IMSC_OFF: isize = 0x38 / 4;
const DMACR_OFF: isize = 0x48 / 4;

const FR_BUSY: u32 = 1 << 3;
const LCR_EN: u32 = 1 << 4;
const LCR_STP2: u32 = 1 << 3;
const CR_UARTEN: u32 = 1 << 0;
const CR_TXEN: u32 = 1 << 8;

static mut UART: UartPL011 = UartPL011 {
    base_addr: core::ptr::null_mut(),
    base_clock: 0,
    baudrate: 0,
    data_bits: 0,
    stop_bits: 0,
}; 

#[unsafe(no_mangle)]
pub extern "C" fn init_uart(base_addr: *mut u32, base_clock: u32, baudrate: u32) {
    unsafe {
       UART = UartPL011 {
           base_addr: base_addr,
           base_clock: base_clock,
           baudrate: baudrate,
           data_bits: 8,
           stop_bits: 1,
       }
    }
}

fn read_reg(offset: isize) -> u32 {
    unsafe {
        core::ptr::read_volatile(UART.base_addr.offset(offset))
    }
}

fn write_reg(offset: isize, val: u32) {
    unsafe {
        core::ptr::write_volatile(UART.base_addr.offset(offset), val);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn configure_uart() {
    // PL011 Sequence
    //1. Disable UART
    write_reg(CR_OFF, 0);

    //2. Wait for current TX to finish
    while(read_reg(FR_OFF) & FR_BUSY) != 0 {}

    //3. Set Baudrate
    unsafe {
        let base_clock = UART.base_clock as u64;
        let baud = UART.baudrate as u64;
        if baud > 0 {
            let baud_div = (base_clock * 1000) / (16 * baud);
            let irbd = (baud_div / 1000) as u32;
            let fbrd = (((baud_div % 1000) * 64 + 500) / 1000) as u32;

            write_reg(IBRD_OFF, irbd);
            write_reg(FBRD_OFF, fbrd);
        }
    }

    //5. Line Control Configuration
    let mut lcr: u32 = 0;
    unsafe {
        lcr |= (((UART.data_bits - 1) as u32 & 0x3) << 5);

        if UART.stop_bits == 2 {
            lcr |= LCR_STP2; 
        }
    }

    lcr |= LCR_EN;

    write_reg(LCR_OFF, lcr);

    write_reg(IMSC_OFF, lcr);

    //6. Mask interrupts
    write_reg(IMSC_OFF, 0);

    //7. Enable uart
    write_reg(CR_OFF, CR_UARTEN | CR_TXEN);

}
 
fn putchar(c: u8) {
    while (read_reg(FR_OFF) & FR_BUSY) != 0 {}
    unsafe {
        core::ptr::write_volatile(UART.base_addr.offset(DR_OFF), c as u32);
    }
}

pub fn print(s: &[u8]) {
    for &c in s {
        putchar(c);
    }
}

