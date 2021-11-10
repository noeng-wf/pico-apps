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

mod freertos;

// The macro for our start-up function
use cortex_m_rt::entry;

// Concurrency (not using FreeRTOS primitives yet)
use core::cell::RefCell;
use cortex_m::interrupt;
use cortex_m::interrupt::Mutex;

// GPIO traits
use embedded_hal::digital::v2::OutputPin;

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

// FFI
use core::ffi::c_void;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();

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
        LED_TASK_INIT_CONTEXT
            .borrow(cs)
            .replace(Some(LedTaskContext {
                pin: led_pin.into(),
            }));
    });

    unsafe {
        let mut task_handle = 0 as freertos::TaskHandle; // not used

        freertos::xTaskCreate(
            led_task,
            "led_task\0".as_ptr(),
            1024,
            0 as *mut c_void,
            1,
            &mut task_handle,
        );
    }

    unsafe {
        freertos::vTaskStartScheduler();
    }

    // Should not be reached
    loop {}
}

struct LedTaskContext {
    pin: DynPin,
}

static LED_TASK_INIT_CONTEXT: Mutex<RefCell<Option<LedTaskContext>>> =
    Mutex::new(RefCell::new(None));

extern "C" fn led_task(_param: *mut c_void) {
    let mut context_option: Option<LedTaskContext> = None;
    interrupt::free(|cs| {
        context_option = LED_TASK_INIT_CONTEXT.borrow(cs).borrow_mut().take();
    });

    if let Some(mut context) = context_option {
        // Blink the LED at 1 Hz
        loop {
            // LED on, and wait for 500ms
            context.pin.set_high().unwrap();
            unsafe {
                freertos::vTaskDelay(500);
            }

            // LED off, and wait for 500ms
            context.pin.set_low().unwrap();
            unsafe {
                freertos::vTaskDelay(500);
            }
        }
    }
}
