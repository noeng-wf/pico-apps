// Skeleton based on Pico Countdown Blinky Example
// from https://github.com/rp-rs/rp-hal/blob/main/boards/pico/examples/pico_countdown_blinky.rs

#![no_std]
#![no_main]

mod cli;
mod display;
mod freertos;
mod text;

use cortex_m_rt::entry;
use pico::hal;
use pico::hal::clocks::Clock;
use pico::hal::pac;

use ds323x::Ds323x;
use ds323x::Hours;
use embedded_time::rate::Extensions;

// Time
use embedded_time::duration::Milliseconds;

use display::data::DOT_MATRIX_WIDTH;
use display::Display;
use text::TextBitmap;

// Program shall halt on panic
use panic_halt as _;

const DISPLAY_TASK_PRIORITY: u32 = 3;
const ANIMATION_TASK_PRIORITY: u32 = 2;
const CLI_TASK_PRIORITY: u32 = 1;

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
    let clocks = hal::clocks::init_clocks_and_plls(
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

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::sio::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut display = Display::new(display::pins::Pins {
        output_disable: pins.gpio13.into(),
        serial_data: pins.gpio11.into(),
        clock: pins.gpio10.into(),
        latch: pins.gpio12.into(),
        address: [pins.gpio16.into(), pins.gpio18.into(), pins.gpio22.into()],
    });

    // Pins for I2C
    let sda_pin = pins.gpio6.into_mode::<hal::gpio::FunctionI2C>();
    let scl_pin = pins.gpio7.into_mode::<hal::gpio::FunctionI2C>();

    let i2c = hal::i2c::I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
    );

    let mut uart = hal::uart::UartPeripheral::<_, _>::enable(
        pac.UART0,
        &mut pac.RESETS,
        hal::uart::common_configs::_115200_8_N_1,
        clocks.peripheral_clock.freq(),
    )
    .unwrap();

    // UART TX (characters sent from RP2040) on pin 1 (GPIO0)
    let _tx_pin = pins.gpio0.into_mode::<hal::gpio::FunctionUart>();
    // UART RX (characters reveived by RP2040) on pin 2 (GPIO1)
    let _rx_pin = pins.gpio1.into_mode::<hal::gpio::FunctionUart>();

    freertos::create_task(
        move || {
            let mut rtc = Ds323x::new_ds3231(i2c);

            let text_bitmap = TextBitmap::from_str("Hello world!").unwrap();
            let mut display_fsm = DisplayFsm::new(&text_bitmap, &mut rtc);

            loop {
                display_fsm.next_step(&mut display);
                freertos::delay(Milliseconds(120));
            }
        },
        &freertos::TaskParameters {
            name: "AnimationTask",
            stack_depth: 1024, // Is actually 4096 bytes because portSTACK_TYPE is uint32_t
            priority: ANIMATION_TASK_PRIORITY,
        },
    );

    freertos::create_task(
        move || {
            cli::run(&mut uart);
        },
        &freertos::TaskParameters {
            name: "CliTask",
            stack_depth: 1024, // Is actually 4096 bytes because portSTACK_TYPE is uint32_t
            priority: CLI_TASK_PRIORITY,
        },
    );

    freertos::start_scheduler();
}

enum DisplayFsmState {
    Time,
    Text,
}

#[derive(PartialEq)]
enum DisplayFsmStateResult {
    Continue,
    Done,
}

struct DisplayFsm<'a, 'b, RtccError> {
    text_bitmap: &'a TextBitmap,
    rtcc: &'b mut dyn ds323x::Rtcc<Error = RtccError>,
    state: DisplayFsmState,
    step: u64,
}

impl<'a, 'b, RtccError> DisplayFsm<'a, 'b, RtccError> {
    fn new(text_bitmap: &'a TextBitmap, rtcc: &'b mut dyn ds323x::Rtcc<Error = RtccError>) -> Self {
        Self {
            text_bitmap,
            rtcc,
            state: DisplayFsmState::Time,
            step: 0,
        }
    }

    fn next_step(&mut self, display: &mut Display) {
        match self.state {
            DisplayFsmState::Time => {
                if self.update_time(display, self.step) == DisplayFsmStateResult::Continue {
                    self.step += 1;
                } else {
                    self.state = DisplayFsmState::Text;
                    self.step = 0;
                }
            }
            DisplayFsmState::Text => {
                if self.update_text(display, self.step) == DisplayFsmStateResult::Continue {
                    self.step += 1;
                } else {
                    self.state = DisplayFsmState::Time;
                    self.step = 0;
                }
            }
        }
    }

    fn update_time(&mut self, display: &mut Display, step: u64) -> DisplayFsmStateResult {
        if let Some((hours, minutes)) = self.get_hours_and_minutes() {
            let mut bitmap = TextBitmap::new();

            // Hours
            if hours >= 10 {
                bitmap.append_char((0x30 + hours / 10) as char).unwrap();
            } else {
                bitmap.append_char(' ').unwrap();
            }
            bitmap.append_char((0x30 + hours % 10) as char).unwrap();

            // Separator
            bitmap.append_char(':').unwrap();

            // Seconds
            bitmap.append_char((0x30 + minutes / 10) as char).unwrap();
            bitmap.append_char((0x30 + minutes % 10) as char).unwrap();

            let bitmap_segment = bitmap.segment(0, DOT_MATRIX_WIDTH);
            let bitmap_data_u32 = bitmap_segment.data.map(|x| x as u32);
            display.modify_data(|x| x.set_dot_matrix(&bitmap_data_u32));
        } else {
            // Show nothing of communication with RTC fails.
            display.modify_data(|x| x.clear());
        }

        if step < 40 {
            DisplayFsmStateResult::Continue
        } else {
            DisplayFsmStateResult::Done
        }
    }

    fn update_text(&mut self, display: &mut Display, step: u64) -> DisplayFsmStateResult {
        let bitmap_offset_min: isize = -(DOT_MATRIX_WIDTH as isize);
        let bitmap_offset_max: isize = self.text_bitmap.width as isize;

        let bitmap_offset = bitmap_offset_min + (step as isize);
        let bitmap_segment = self.text_bitmap.segment(bitmap_offset, DOT_MATRIX_WIDTH);
        let bitmap_data_u32 = bitmap_segment.data.map(|x| x as u32);
        display.modify_data(|x| x.set_dot_matrix(&bitmap_data_u32));

        if bitmap_offset < bitmap_offset_max {
            DisplayFsmStateResult::Continue
        } else {
            DisplayFsmStateResult::Done
        }
    }

    fn get_hours_and_minutes(&mut self) -> Option<(u8, u8)> {
        let hours;
        let minutes;

        if let Ok(value) = self.rtcc.get_hours() {
            match value {
                Hours::AM(x) => {
                    hours = x;
                }
                Hours::PM(x) => {
                    hours = x;
                }
                Hours::H24(x) => {
                    hours = x;
                }
            }
        } else {
            return None;
        }

        if let Ok(value) = self.rtcc.get_minutes() {
            minutes = value;
        } else {
            return None;
        }

        Some((hours, minutes))
    }
}
