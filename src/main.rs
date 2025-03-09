#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Define a minimal entry point (instead of main)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
