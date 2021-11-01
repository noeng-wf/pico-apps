// Skeleton based on Pico Countdown Blinky Example
// from https://github.com/rp-rs/rp-hal/blob/main/boards/pico/examples/pico_countdown_blinky.rs

#![no_std]
#![no_main]

mod display;
mod text;

use cortex_m_rt::entry;
use pico::hal;
use pico::hal::pac;

use display::data::DOT_MATRIX_WIDTH;
use display::Display;
use text::TextBitmap;

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

    // Pins for the LED display
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

    let mut scan_cycle = CycleGenerator::new(&timer, 1000);
    let mut animation_cycle = CycleGenerator::new(&timer, 120000);
    let mut animation_step = 0;

    let bitmap = TextBitmap::from_str("Hello world!").unwrap();

    loop {
        if animation_cycle.is_elapsed() {
            // Note:
            // This should only be called if the step counter actually changes to avoid too much CPU load
            // making visible varation of the scan cycle duration (causing slight LED flickering).
            apply_display_step(&mut display, &bitmap, animation_step);
            animation_step += 1;
        }

        if scan_cycle.is_elapsed() {
            display.do_scan_cycle();
        }
    }
}

fn apply_display_step(display: &mut Display, bitmap: &TextBitmap, step: u64) {
    let bitmap_offset_min: isize = -(DOT_MATRIX_WIDTH as isize);
    let bitmap_offset_max: isize = bitmap.width as isize;

    let bitmap_offset =
        bitmap_offset_min + (step % ((bitmap_offset_max - bitmap_offset_min + 1) as u64)) as isize;
    let bitmap_segment = bitmap.segment(bitmap_offset, DOT_MATRIX_WIDTH);
    let bitmap_data_u32 = bitmap_segment.data.map(|x| x as u32);
    display.data.set_dot_matrix(&bitmap_data_u32);
}

struct CycleGenerator<'a> {
    timer: &'a hal::timer::Timer,
    period_us: u64,
    next_counter_us: u64,
}

impl<'a> CycleGenerator<'a> {
    fn new(timer: &'a hal::timer::Timer, period_us: u64) -> Self {
        Self {
            timer,
            period_us,
            next_counter_us: timer.get_counter(),
        }
    }

    fn is_elapsed(&mut self) -> bool {
        let counter_us = self.timer.get_counter();

        if counter_us >= self.next_counter_us {
            self.next_counter_us += self.period_us;
            true
        } else {
            false
        }
    }
}
