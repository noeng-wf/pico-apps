#![allow(dead_code)] // Not all functionality here must be used.

pub const RAW_WIDTH: usize = 32; // but only the lower 24 bits are connected to a column
pub const RAW_HEIGHT: usize = 8;
pub type RawData = [u32; RAW_HEIGHT];

pub const DOT_MATRIX_WIDTH: usize = 22;
pub const DOT_MATRIX_HEIGHT: usize = 7;
pub type DotMatrixData = [u32; DOT_MATRIX_HEIGHT];

pub enum Indicator {
    Mon,
    Tues,
    Wed,
    Thur,
    Fri,
    Sat,
    Sun,
    MoveOn,
    AlarmOn,
    CountDown,
    DegreeC,
    DegreeF,
    AM,
    PM,
    CountUp,
    Hourly,
    AutoLight,
}

pub struct Data {
    pub raw_data: RawData,
}

impl Data {
    pub fn new() -> Self {
        Self {
            raw_data: [0; RAW_HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.raw_data = [0; RAW_HEIGHT];
    }

    pub fn set_dot_matrix(&mut self, data: &DotMatrixData) {
        const DOT_MATRIX_X_OFFSET: usize = 2;
        const DOT_MATRIX_Y_OFFSET: usize = 1;

        const DATA_MASK: u32 = (1 << DOT_MATRIX_WIDTH) - 1;
        const RAW_DATA_MASK: u32 = DATA_MASK << DOT_MATRIX_X_OFFSET;

        for row in 0..DOT_MATRIX_HEIGHT {
            // Ensure no other bits are set
            assert!(data[row] & !DATA_MASK == 0);

            let raw_row_data = &mut self.raw_data[row + DOT_MATRIX_Y_OFFSET];
            *raw_row_data = *raw_row_data & !RAW_DATA_MASK | (data[row] << DOT_MATRIX_X_OFFSET);
        }
    }

    pub fn set_indicator(&mut self, indicator: Indicator, state: bool) {
        let (row, mask) = match indicator {
            Indicator::Mon => (0, 0x00000018),
            Indicator::Tues => (0, 0x000000C0),
            Indicator::Wed => (0, 0x00000600),
            Indicator::Thur => (0, 0x00003000),
            Indicator::Fri => (0, 0x00018000),
            Indicator::Sat => (0, 0x000C0000),
            Indicator::Sun => (0, 0x00600000),
            Indicator::MoveOn => (0, 0x00000003),
            Indicator::AlarmOn => (1, 0x00000003),
            Indicator::CountDown => (2, 0x00000003),
            Indicator::DegreeF => (3, 0x00000001),
            Indicator::DegreeC => (3, 0x00000002),
            Indicator::AM => (4, 0x00000001),
            Indicator::PM => (4, 0x00000002),
            Indicator::CountUp => (5, 0x00000003),
            Indicator::Hourly => (6, 0x00000003),
            Indicator::AutoLight => (7, 0x00000003),
        };

        let raw_row_data = &mut self.raw_data[row];
        *raw_row_data = *raw_row_data & !mask | (if state { mask } else { 0 });
    }
}
