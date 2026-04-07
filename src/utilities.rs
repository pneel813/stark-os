use core::ptr::{read_volatile, write_volatile};

pub unsafe fn read_reg(base: usize, offset: usize) -> u32 {
    unsafe {
        let ptr = (base as *const u8).add(offset) as *const u32;
        return read_volatile(ptr);
    }
}

pub unsafe fn write_reg(base: usize, offset: usize, value: u32) {
    unsafe {
        let ptr = (base as *mut u8).add(offset) as *mut u32;
        write_volatile(ptr, value);
    }
}

pub unsafe fn set_reg_bits(base: usize, offset: usize, bits: u32) {
    unsafe {
        let ptr = (base as *mut u8).add(offset) as *mut u32;
        let current_val = read_volatile(ptr);
        write_volatile(ptr, current_val | bits);
    }
}

pub unsafe fn clear_reg_bits(base: usize, offset: usize, bits: u32) {
    unsafe {
        let ptr = (base as *mut u8).add(offset) as *mut u32;
        let current_val = read_volatile(ptr);
        write_volatile(ptr, current_val & !bits);
    }
}
