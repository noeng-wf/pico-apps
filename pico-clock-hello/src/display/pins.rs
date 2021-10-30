//! Set of pins to be used for the display.

use core::convert::Infallible;
use embedded_hal::digital::v2::OutputPin;

type OutPinRef<'a> = &'a mut dyn OutputPin<Error = Infallible>;

/// Reference to the output pins.
pub struct Pins<'a> {
    // /OE
    pub output_disable: OutPinRef<'a>,
    // SDI
    pub serial_data: OutPinRef<'a>,
    // CLK
    pub clock: OutPinRef<'a>,
    // LE
    pub latch: OutPinRef<'a>,
    // A0..A2
    pub address: [OutPinRef<'a>; 3],
}
