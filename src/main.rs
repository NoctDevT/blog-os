
#![no_std] // No standard lib
#![no_main] // disable all Rust level entry points

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}

