#![no_std]
#![no_main]
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let hello = b"Happy me!!!";
    let buffer = 0xb8000 as *mut u8;
    for (i, &byte) in hello.iter().enumerate() {
        unsafe {
            *buffer.offset(i as isize * 2) = byte;
            *buffer.offset(i as isize * 2 + 1) = 0x0f;
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
