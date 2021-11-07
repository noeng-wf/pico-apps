// Code copied from https://github.com/rp-rs/rp-hal/blob/main/boards/pico/examples/pico_countdown_blinky.rs

//! # Pico Countdown Blinky Example
//!
//! Blinks the LED on a Pico board, using an RP2040 Timer in Count-down mode.
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for
//! the on-board LED.
//!
//! See the `Cargo.toml` file for Copyright and licence details.

#![no_std]
#![no_main]

// Attributes for special functions
use cortex_m_rt::{entry, exception};

// Interrupt handler concurrency
use core::cell::RefCell;
use cortex_m::interrupt;
use cortex_m::interrupt::Mutex;

// GPIO traits
use embedded_hal::digital::v2::{OutputPin, PinState};

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use pico::hal;
use pico::hal::gpio::dynpin::DynPin;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

struct SysTickContext {
    led_pin: DynPin,
    period_ms: u32,
    counter_ms: u32,
}

static SYS_TICK_INIT_CONTEXT: Mutex<RefCell<Option<SysTickContext>>> =
    Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let mut core_pac = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::watchdog::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
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

    // Initialize SysTick interrupt with 1ms period
    core_pac.SYST.set_reload(1000); // 1000us
    core_pac.SYST.clear_current();
    core_pac.SYST.enable_counter();
    core_pac.SYST.enable_interrupt();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::sio::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let led_pin = pins.led.into_push_pull_output();
    interrupt::free(|cs| {
        SYS_TICK_INIT_CONTEXT
            .borrow(cs)
            .replace(Some(SysTickContext {
                led_pin: led_pin.into(),
                period_ms: 500,
                counter_ms: 0,
            }));
    });

    loop {}
}

#[exception]
fn SysTick() {
    static mut CONTEXT: Option<SysTickContext> = None;

    if CONTEXT.is_none() {
        interrupt::free(|cs| {
            if let Some(init_context) = SYS_TICK_INIT_CONTEXT.borrow(cs).borrow_mut().take() {
                CONTEXT.replace(init_context);
            }
        });
    }

    if let Some(context) = CONTEXT {
        context
            .led_pin
            .set_state(if (context.counter_ms / context.period_ms) & 1 == 0 {
                PinState::High
            } else {
                PinState::Low
            })
            .unwrap();
        context.counter_ms += 1;
    }
}
