// Skeleton based on Pico Countdown Blinky Example
// from https://github.com/rp-rs/rp-hal/blob/main/boards/pico/examples/pico_countdown_blinky.rs

#![no_std]
#![no_main]

mod display;

use cortex_m_rt::entry;
use pico::hal;
use pico::hal::pac;

use display::data::Indicator;
use display::Display;

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

    let mut output_disable_pin = pins.gpio13.into_push_pull_output();
    let mut serial_data_pin = pins.gpio11.into_push_pull_output();
    let mut clock_pin = pins.gpio10.into_push_pull_output();
    let mut latch_pin = pins.gpio12.into_push_pull_output();
    let mut address0_pin = pins.gpio16.into_push_pull_output();
    let mut address1_pin = pins.gpio18.into_push_pull_output();
    let mut address2_pin = pins.gpio22.into_push_pull_output();

    let mut display = Display::new(display::pins::Pins {
        output_disable: &mut output_disable_pin,
        serial_data: &mut serial_data_pin,
        clock: &mut clock_pin,
        latch: &mut latch_pin,
        address: [&mut address0_pin, &mut address1_pin, &mut address2_pin],
    });
    display.data.raw_data = [0xFFFFFFFF; 8];

    const SCAN_CYCLE_US: u64 = 1000;
    let mut next_scan_counter_us: Option<u64> = None;
    loop {
        let counter_us = timer.get_counter();
        let step = timer.get_counter() / 300000;

        display.data.clear();

        let indicator = match step % 17 {
            0 => Indicator::Mon,
            1 => Indicator::Tues,
            2 => Indicator::Wed,
            3 => Indicator::Thur,
            4 => Indicator::Fri,
            5 => Indicator::Sat,
            6 => Indicator::Sun,
            7 => Indicator::MoveOn,
            8 => Indicator::AlarmOn,
            9 => Indicator::CountDown,
            10 => Indicator::DegreeF,
            11 => Indicator::DegreeC,
            12 => Indicator::AM,
            13 => Indicator::PM,
            14 => Indicator::CountUp,
            15 => Indicator::Hourly,
            16 | _ => Indicator::AutoLight,
        };
        display.data.set_indicator(indicator, true);

        let dot_matrix_data: display::data::DotMatrixData = [
            0x0F0F0F0F & 0x003FFFFF,
            0x1E1E1E1E & 0x003FFFFF,
            0x3C3C3C3C & 0x003FFFFF,
            0x78787878 & 0x003FFFFF,
            0xF0F0F0F0 & 0x003FFFFF,
            0xE1E1E1E1 & 0x003FFFFF,
            0xC3C3C3C3 & 0x003FFFFF,
        ];
        display.data.set_dot_matrix(&dot_matrix_data);

        if let Some(x) = next_scan_counter_us {
            if counter_us >= x {
                display.do_scan_cycle();
                next_scan_counter_us = Some(x + SCAN_CYCLE_US);
            }
        } else {
            display.do_scan_cycle();
            next_scan_counter_us = Some(counter_us + SCAN_CYCLE_US);
        }
    }
}
