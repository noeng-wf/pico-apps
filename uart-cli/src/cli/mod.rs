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

fn process_line<T: Write>(uart: &mut T, line: &str) {
    let line = line.trim();
    if line.len() == 0 {
        return;
    }

    let mut iter = line.split_whitespace();
    match iter.next().unwrap() {
        "print" => {
            loop {
                if let Some(word) = iter.next() {
                    write!(uart, "{} ", word).unwrap();
                } else {
                    break;
                }
            }
            print_newline(uart);
        }
        "help" => print_help(uart),
        _ => {
            write!(uart, "Unknown command\r\n").unwrap();
            print_newline(uart);
            print_help(uart);
        }
    }
}

fn print_help<T: Write>(uart: &mut T) {
    write!(uart, "Supported commands:\r\n").unwrap();
    write!(uart, "  help            Print this help\r\n").unwrap();
    write!(uart, "  print <words>   Print the given words\r\n").unwrap();
}
