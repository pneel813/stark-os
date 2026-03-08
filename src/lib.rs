#![no_main]
#![no_std]

mod uart;
mod expections;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn kmain() {
    uart::print(b"Stark OS v1.0!\n");
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    uart::print(b"Panic!\n");
    loop {}
}
