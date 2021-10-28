use core::convert::Infallible;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::digital::v2::PinState;
use pico::hal;

type OutPinRef<'a> = &'a mut dyn OutputPin<Error = Infallible>;

const SCAN_CYCLE_US: u64 = 1000;
const ANIMATION_CYCLE_US: u64 = 200000;

pub struct DotMatrix<'a> {
    // OE
    output_enable_pin: OutPinRef<'a>,
    // SDI
    serial_data_pin: OutPinRef<'a>,
    // CLK
    clock_pin: OutPinRef<'a>,
    // LE
    latch_pin: OutPinRef<'a>,
    // A0..A2
    address_pins: [OutPinRef<'a>; 3],

    timer: &'a hal::timer::Timer,

    next_counter: Option<u64>,
    row: usize,
}

impl<'a> DotMatrix<'a> {
    pub fn new(
        output_enable_pin: OutPinRef<'a>,
        serial_data_pin: OutPinRef<'a>,
        clock_pin: OutPinRef<'a>,
        latch_pin: OutPinRef<'a>,
        address_pins: [OutPinRef<'a>; 3],
        timer: &'a hal::timer::Timer,
    ) -> Self {
        output_enable_pin.set_high().unwrap();

        DotMatrix {
            output_enable_pin,
            serial_data_pin,
            clock_pin,
            latch_pin,
            address_pins,
            timer,
            next_counter: None,
            row: 0
        }
    }

    pub fn run(&mut self) {
        let current_counter = self.timer.get_counter();
        match self.next_counter {
            Some(x) => {
                if current_counter < x {
                    return;
                } else {
                    self.next_counter = Some(x + SCAN_CYCLE_US);
                }
            },
            None => {
                self.next_counter = Some(current_counter + SCAN_CYCLE_US);
            }
        }

        let pattern_offset = ((current_counter / ANIMATION_CYCLE_US) % 8) as usize;

        self.output_enable_pin.set_high().unwrap();
        self.select_row(self.row);
        self.write_row(rotate_bits(0x07070707, self.row + pattern_offset));
        //self.write_row(0x0F0F0F0F << self.row);
        self.output_enable_pin.set_low().unwrap();

        self.row = (self.row + 1) % 8;
    }

    fn select_row(&mut self, row: usize) {
        assert!(row < 8);
        self.address_pins[0].set_state(PinState::from((row & 1) != 0)).unwrap();
        self.address_pins[1].set_state(PinState::from((row & 2) != 0)).unwrap();
        self.address_pins[2].set_state(PinState::from((row & 4) != 0)).unwrap();
    }

    fn write_row(&mut self, mut row_bits: u32) {
        for _ in 0..32 {
            self.clock_pin.set_low().unwrap();
            self.serial_data_pin.set_state(PinState::from((row_bits & 1) != 0)).unwrap();
            self.clock_pin.set_high().unwrap();

            row_bits = row_bits >> 1;
        }

        self.latch_pin.set_high().unwrap();
        self.latch_pin.set_low().unwrap();
    }
}

fn rotate_bits(value: u32, offset: usize) -> u32 {
    let x = offset % 32;

    if x > 0 {
        value << (offset % 32) | value >> (32 - (offset % 32))
    } else {
        // prevent overflow panic from shifting right by 32
        value
    }
}
