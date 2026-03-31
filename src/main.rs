#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

#![reexport_test_harness_main = "test_main"]
#[cfg(test)]
pub fn test_runner(tests: [fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

use core::panic::PanicInfo;

mod vga_buffer;

// #[unsafe(no_mangle)]
// pub extern "C" fn _start() -> ! {
//     println!("Hello World{}", "!");
//
//     loop {}
// }

/// This function is called on panic.
/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("Yo, there's an error somewhere...");
    loop {}
}



#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}