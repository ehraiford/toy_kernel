#![no_std]
#![no_main]

use core::panic::PanicInfo;
use toy_kernel::{QemuExitCode, exit_qemu, init, serial_println};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init();
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn should_fail() {
    toy_kernel::serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}
