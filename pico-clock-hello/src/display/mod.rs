//! Driver for the dot matrix LED display of the "Pico Clock Green" kit:
//! https://www.waveshare.com/pico-clock-green.htm

pub mod data;
pub mod pins;

use embedded_hal::digital::v2::PinState;

use crate::display::data::{Data, RAW_HEIGHT, RAW_WIDTH};
use crate::display::pins::Pins;

/// Abstraction of the dot matrix LED display.
pub struct Display<'a> {
    pins: Pins<'a>,
    pub data: Data,

    scan_row: usize,
}

impl<'a> Display<'a> {
    pub fn new(pins: Pins<'a>) -> Self {
        // Disable output by default
        pins.output_disable.set_high().unwrap();

        Self {
            pins,
            data: Data::new(),
            scan_row: 0,
        }
    }

    /// Has to be called periodically (frequency: refresh rate multiplied by 8 rows)
    pub fn do_scan_cycle(&mut self) {
        self.pins.output_disable.set_high().unwrap();
        self.select_row(self.scan_row);
        self.write_row(self.data.raw_data[self.scan_row]);
        self.pins.output_disable.set_low().unwrap();

        self.scan_row = (self.scan_row + 1) % RAW_HEIGHT;
    }

    fn select_row(&mut self, row: usize) {
        assert!(row < RAW_HEIGHT);
        self.pins.address[0]
            .set_state(PinState::from((row & 1) != 0))
            .unwrap();
        self.pins.address[1]
            .set_state(PinState::from((row & 2) != 0))
            .unwrap();
        self.pins.address[2]
            .set_state(PinState::from((row & 4) != 0))
            .unwrap();
    }

    fn write_row(&mut self, mut raw_data: u32) {
        for _ in 0..RAW_WIDTH {
            self.pins.clock.set_low().unwrap();
            self.pins
                .serial_data
                .set_state(PinState::from((raw_data & 1) != 0))
                .unwrap();
            self.pins.clock.set_high().unwrap();

            raw_data >>= 1;
        }

        self.pins.latch.set_high().unwrap();
        self.pins.latch.set_low().unwrap();
    }
}
