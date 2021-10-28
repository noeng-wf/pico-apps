// Skeleton based on Pico Countdown Blinky Example
// from https://github.com/rp-rs/rp-hal/blob/main/boards/pico/examples/pico_countdown_blinky.rs

#![no_std]
#![no_main]

mod dot_matrix;

use cortex_m_rt::entry;
use pico::hal;
use pico::hal::pac;

use dot_matrix::DotMatrix;

// Program shall halt on panic
use panic_halt as _;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[entry]
fn main() -> ! {
    // Peripherals
    let mut pac = pac::Peripherals::take().unwrap();
    // Watchdog driver needed for clock setup
    let mut watchdog = hal::watchdog::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    // The default is to generate a 125 MHz system clock
    let _clocks = hal::clocks::init_clocks_and_plls(
        pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Configure the Timer peripheral in count-down mode
    let timer = hal::timer::Timer::new(pac.TIMER, &mut pac.RESETS);

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::sio::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut output_enable_pin = pins.gpio13.into_push_pull_output();
    let mut serial_data_pin = pins.gpio11.into_push_pull_output();
    let mut clock_pin = pins.gpio10.into_push_pull_output();
    let mut latch_pin = pins.gpio12.into_push_pull_output();
    let mut address0_pin = pins.gpio16.into_push_pull_output();
    let mut address1_pin = pins.gpio18.into_push_pull_output();
    let mut address2_pin = pins.gpio22.into_push_pull_output();

    let mut dot_matrix = DotMatrix::new(
        &mut output_enable_pin,
        &mut serial_data_pin,
        &mut clock_pin,
        &mut latch_pin,
        [&mut address0_pin, &mut address1_pin, &mut address2_pin],
        &timer,
    );

    loop {
        dot_matrix.run();
    }
}
