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
}

#[unsafe(no_mangle)]
pub extern "C" fn init_uart(base_addr: *mut u32, base_clock: u32, baudrate: u32) {
    unsafe {
        
    }
}
