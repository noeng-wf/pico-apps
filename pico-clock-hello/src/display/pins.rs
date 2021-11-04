//! Set of pins to be used for the display.

use pico::hal::gpio::dynpin::DynPin;

/// Output pins.
pub struct Pins {
    // /OE
    pub output_disable: DynPin,
    // SDI
    pub serial_data: DynPin,
    // CLK
    pub clock: DynPin,
    // LE
    pub latch: DynPin,
    // A0..A2
    pub address: [DynPin; 3],
}
