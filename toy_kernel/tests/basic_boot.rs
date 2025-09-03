#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(toy_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use toy_kernel::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[test_case]
fn test_println() {
    println!("test_println output");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
