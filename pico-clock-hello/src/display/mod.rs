//! Driver for the dot matrix LED display of the "Pico Clock Green" kit:
//! https://www.waveshare.com/pico-clock-green.htm

pub mod data;
pub mod pins;

use embedded_hal::digital::v2::OutputPin;
use embedded_hal::digital::v2::PinState;

// Interrupt handler concurrency
use core::cell::RefCell;
use cortex_m::interrupt;
use cortex_m::interrupt::Mutex;

use crate::display::data::{Data, RawData, RAW_HEIGHT, RAW_WIDTH};
use crate::display::pins::Pins;

pub struct SysTickContext {
    pins: Option<Pins>,
    scan_row: usize,
}

impl SysTickContext {
    pub const fn new() -> Self {
        Self {
            pins: None,
            scan_row: 0,
        }
    }
}

struct SysTickInit {
    pins: Pins,
}

static SYS_TICK_INIT: Mutex<RefCell<Option<SysTickInit>>> = Mutex::new(RefCell::new(None));

static SYS_TICK_DATA: Mutex<RefCell<RawData>> = Mutex::new(RefCell::new([0; RAW_HEIGHT]));

/// Abstraction of the dot matrix LED display.
pub struct Display {
    data: Data,
}

impl Display {
    pub fn new(mut pins: Pins) -> Self {
        pins.output_disable.into_push_pull_output();
        pins.serial_data.into_push_pull_output();
        pins.clock.into_push_pull_output();
        pins.latch.into_push_pull_output();
        pins.address
            .iter_mut()
            .for_each(|x| x.into_push_pull_output());

        // Disable output by default
        pins.output_disable.set_high().unwrap();

        // Initialization context for SysTick handler (variables to be moved into the handler)
        interrupt::free(|cs| {
            SYS_TICK_INIT.borrow(cs).replace(Some(SysTickInit { pins }));
        });

        Self { data: Data::new() }
    }

    pub fn modify_data<F>(&mut self, func: F)
    where
        F: FnOnce(&mut Data),
    {
        func(&mut self.data);
        interrupt::free(|cs| SYS_TICK_DATA.borrow(cs).replace(self.data.raw_data));
    }

    /// Will be periodically from SysTick interrupt (frequency: refresh rate multiplied by 8 rows)
    ///
    /// Notes:
    /// The context argument allows storing context data in the interrupt handler to avoid marking code
    /// in this module as unsafe because of otherwise required local 'static mut' variables
    /// in on_sys_tick_interrupt().
    /// The 'expection' attribute on the interrupt handler does some magic so this problem doesn't
    /// occur there.
    pub fn on_sys_tick_interrupt(context: &mut SysTickContext) {
        // Initialize pins if required
        if context.pins.is_none() {
            interrupt::free(|cs| {
                if let Some(init) = SYS_TICK_INIT.borrow(cs).borrow_mut().take() {
                    context.pins.replace(init.pins);
                }
            });
        }

        if let Some(pins) = context.pins.as_mut() {
            let mut raw_data: u32 = 0;
            interrupt::free(|cs| {
                raw_data = SYS_TICK_DATA.borrow(cs).borrow()[context.scan_row];
            });

            pins.output_disable.set_high().unwrap();
            Display::select_row(pins, context.scan_row);
            Display::write_row(pins, raw_data);
            pins.output_disable.set_low().unwrap();

            context.scan_row = (context.scan_row + 1) % RAW_HEIGHT;
        }
    }

    fn select_row(pins: &mut Pins, row: usize) {
        assert!(row < RAW_HEIGHT);
        pins.address[0]
            .set_state(PinState::from((row & 1) != 0))
            .unwrap();
        pins.address[1]
            .set_state(PinState::from((row & 2) != 0))
            .unwrap();
        pins.address[2]
            .set_state(PinState::from((row & 4) != 0))
            .unwrap();
    }

    fn write_row(pins: &mut Pins, mut raw_data: u32) {
        for _ in 0..RAW_WIDTH {
            pins.clock.set_low().unwrap();
            pins.serial_data
                .set_state(PinState::from((raw_data & 1) != 0))
                .unwrap();
            pins.clock.set_high().unwrap();

            raw_data >>= 1;
        }

        pins.latch.set_high().unwrap();
        pins.latch.set_low().unwrap();
    }
}
