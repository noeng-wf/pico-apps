pub enum LineInputResult<'a> {
    None,
    Echo(u8),
    Complete(&'a [u8]),
}

pub struct LineInput<const MAX_LEN: usize> {
    data: [u8; MAX_LEN],
    len: usize,
}

impl<'a, const MAX_LEN: usize> LineInput<MAX_LEN> {
    pub fn new() -> Self {
        Self {
            data: [0; MAX_LEN],
            len: 0,
        }
    }

    pub fn feed(&'a mut self, c: u8) -> LineInputResult<'a> {
        match c {
            // Backspace
            0x7F => {
                if self.len > 0 {
                    self.len -= 1;
                    LineInputResult::Echo(c)
                } else {
                    LineInputResult::None
                }
            }
            // Return
            b'\r' => {
                if self.len > 0 {
                    let line = &self.data[0..self.len];
                    self.len = 0;
                    LineInputResult::Complete(line)
                } else {
                    LineInputResult::None
                }
            }
            // Printable character
            0x20..=0x7E => {
                if self.len < self.data.len() {
                    self.data[self.len] = c;
                    self.len += 1;
                    LineInputResult::Echo(c)
                } else {
                    LineInputResult::None
                }
            }
            _ => LineInputResult::None,
        }
    }
}
