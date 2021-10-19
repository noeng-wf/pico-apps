#![no_std]

use core::panic::PanicInfo;

#[no_mangle]
pub extern fn get_delay_value() -> u32 {
    500
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
