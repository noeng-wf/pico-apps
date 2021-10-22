#![no_std]
#![no_main]

mod blink;

use cortex_m_rt::entry;
use rp2040_pac::Peripherals;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

fn switch_to_xosc(p: &mut Peripherals) {
    // Enable crystal oscillator
    p.XOSC.ctrl.modify(|_r, w| w.enable().enable());
    // Wait until crystal oscillator stable
    while p.XOSC.status.read().stable().bit_is_clear() {}

    // Switch clk_ref to crystal oscillator (12 MHz)
    p.CLOCKS.clk_ref_ctrl.modify(|_r, w| w.src().xosc_clksrc());
    // Divisor = 1
    p.CLOCKS.clk_ref_div.write(|w| unsafe { w.int().bits(1) });

    // Enable 1 us tick generation (dividing clk_ref by 12)
    p.WATCHDOG.tick.write(|w| unsafe { w.enable().set_bit().cycles().bits(12) });
}

fn start_subsystems(p: &mut Peripherals) {
    // Take used subsystems out of reset state
    p.RESETS.reset.modify(|_r, w| w.pio0().clear_bit().pads_bank0().clear_bit().io_bank0().clear_bit());

    // Wait until reset state has been left
    loop {
        let value = p.RESETS.reset_done.read();
        if value.pio0().bit_is_set() && value.pads_bank0().bit_is_set() && value.io_bank0().bit_is_set() {
            break;
        }
    }
}

#[entry]
fn main() -> ! {
    let mut p = Peripherals::take().unwrap();
    switch_to_xosc(&mut p);
    start_subsystems(&mut p);

    blink::blink_loop(&mut p);
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
