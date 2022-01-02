mod line_input;
mod arguments;

use embedded_hal::serial::Read as HalRead;
use embedded_hal::serial::Write as HalWrite;

// Time
use embedded_time::duration::Milliseconds;

// Some traits we need
use core::fmt::Write;
use core::result::Result;
use nb::block;

use line_input::{LineInput, LineInputResult};

use arguments::{ArgumentError, split_first_argument};

const MAX_LINE_LENGTH: usize = 100;

pub trait Timer {
    fn sleep_ms(&self, delay_ms: u32);
}

pub fn run<T: HalRead<u8> + HalWrite<u8> + Write>(uart: &mut T) -> ! {
    let mut input = LineInput::<MAX_LINE_LENGTH>::new();

    print_prompt(uart);

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
                        print_newline(uart);
                        process_line(uart, line);
                        print_prompt(uart);
                    }
                }
            }
            Result::Err(nb::Error::WouldBlock) => {
                // Nothing received: Continue polling
                crate::freertos::delay(Milliseconds(1)); // Precaution because it is still busy waiting
            }
            Result::Err(nb::Error::Other(_)) => {
                // Ignore UART errors
            }
        }
    }
}

fn print_prompt<T: Write>(uart: &mut T) {
    write!(uart, "> ").unwrap();
}

fn print_newline<T: Write>(uart: &mut T) {
    write!(uart, "\r\n").unwrap();
}

fn process_line<T: Write>(uart: &mut T, line: &str) -> Result<(), ArgumentError> {
    let line = line.trim(); // ensure that split_first_argument returns empty string after last argument
    let mut buffer: [u8; MAX_LINE_LENGTH] = [0; MAX_LINE_LENGTH];
    let (command, remaining_line) = split_first_argument(line, &mut buffer)?;
    match command {
        "" => {
            return Ok(());
        },
        "settext" => {
            process_settext_command(uart, remaining_line)?;
        },
        "settime" => {
            process_settime_command(uart, remaining_line)?;
        },
        "help" => print_help(uart),
        _ => {
            write!(uart, "Unknown command\r\n").unwrap();
            print_newline(uart);
            print_help(uart);
        }
    }

    Ok(())
}

fn process_settext_command<T: Write>(uart: &mut T, _line: &str) -> Result<(), ArgumentError> {
    write!(uart, "To be implemented\r\n").unwrap();
    Ok(())
}

fn process_settime_command<T: Write>(uart: &mut T, _line: &str) -> Result<(), ArgumentError> {
    write!(uart, "To be implemented\r\n").unwrap();
    Ok(())
}

fn print_help<T: Write>(uart: &mut T) {
    write!(uart, "Supported commands:\r\n").unwrap();
    write!(uart, "  help            Print this help\r\n").unwrap();
    write!(
        uart,
        "  settime <time>  Set the time (format: 'HH:MM:SS')\r\n"
    )
    .unwrap();
    write!(
        uart,
        "  settext <text>  Set the scrolling text (without leading and trailing whitespace)\r\n"
    )
    .unwrap();
}
