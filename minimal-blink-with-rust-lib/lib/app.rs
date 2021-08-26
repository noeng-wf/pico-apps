#![no_std]

use core::panic::PanicInfo;

static VALUES: [u32; 10] = [ 0, 0, 1, 2, 4, 10, 10, 4, 2, 1 ];
static mut INDEX: usize = 0;

#[no_mangle]
pub extern fn get_next_dim_value() -> u32 {
   unsafe {
      let value = VALUES[INDEX];
      INDEX = (INDEX + 1) % VALUES.len();
      value
   }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
