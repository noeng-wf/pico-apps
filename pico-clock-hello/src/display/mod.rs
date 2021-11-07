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

struct SysTickContext {
    pins: Pins,
    scan_row: usize,
}

static SYS_TICK_INIT_CONTEXT: Mutex<RefCell<Option<SysTickContext>>> =
    Mutex::new(RefCell::new(None));

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
            SYS_TICK_INIT_CONTEXT
                .borrow(cs)
                .replace(Some(SysTickContext { pins, scan_row: 0 }));
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
    pub fn on_sys_tick_interrupt() {
        static mut CONTEXT: Option<SysTickContext> = None;

        // Initialize context if required
        if unsafe { CONTEXT.is_none() } {
            interrupt::free(|cs| {
                if let Some(init_context) = SYS_TICK_INIT_CONTEXT.borrow(cs).borrow_mut().take() {
                    unsafe {
                        CONTEXT.replace(init_context);
                    }
                }
            });
        }

        if let Some(context) = unsafe { CONTEXT.as_mut() } {
            let mut raw_data: u32 = 0;
            interrupt::free(|cs| {
                raw_data = SYS_TICK_DATA.borrow(cs).borrow()[context.scan_row];
            });

            context.pins.output_disable.set_high().unwrap();
            Display::select_row(&mut context.pins, context.scan_row);
            Display::write_row(&mut context.pins, raw_data);
            context.pins.output_disable.set_low().unwrap();

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
