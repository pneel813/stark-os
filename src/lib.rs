#![no_main]
#![no_std]

mod expections;
mod gic;
mod uart;
mod utilities;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn kmain() {
    uart::print(b"Hello, from Rust\n");
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    uart::print(b"Panic!\n");
    loop {}
}
