mod line_input;

use embedded_hal::serial::Read as HalRead;
use embedded_hal::serial::Write as HalWrite;

// Some traits we need
use core::fmt::Write;
use core::result::Result;
use nb::block;

use line_input::{LineInput, LineInputResult};

pub trait Timer {
    fn sleep_ms(&self, delay_ms: u32);
}

pub fn run<T: HalRead<u8> + HalWrite<u8> + Write>(uart: &mut T, _timer: &impl Timer) -> ! {
    let mut input = LineInput::<100>::new();

    loop {
        match uart.read() {
            Result::Ok(c) => {
                match input.feed(c) {
                    LineInputResult::None => {
                        // No echo
                    }
                    LineInputResult::Echo(c) => {
                        block!(uart.write(c)).unwrap_or(());
                    }
                    LineInputResult::Complete(line) => {
                        write!(uart, "\r\n").unwrap();
                        process_line(uart, line)
                    }
                }
            }
            Result::Err(nb::Error::WouldBlock) => {
                // Nothing received: Continue polling
            }
            Result::Err(nb::Error::Other(_)) => {
                // Ignore UART errors
            }
        }
    }
}

fn process_line<T: Write>(uart: &mut T, line: &[u8]) {
    let line = core::str::from_utf8(line).unwrap();
    write!(uart, "Got a line: {}\r\n", line).unwrap();
}
