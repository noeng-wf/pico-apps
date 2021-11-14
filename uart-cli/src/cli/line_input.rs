//! Converts character input from a serial line into lines of type 'str'.

pub enum LineInputResult<'a> {
    /// Character not accepted. Not echo shall be done.
    None,
    /// Caller should echo the given character (usually the same as entered).
    Echo(u8),
    /// Caller receives a complete line.
    /// Note: The caller is responsible to echo a new line.
    Complete(&'a str),
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

    /// Notifies about an entered char.
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
                // Note: It is also allowed to enter empty lines.

                // Conversion to UTF8 string should not fail because only ASCII chars have been accepted.
                let line = core::str::from_utf8(&self.data[0..self.len]).unwrap();
                self.len = 0;
                LineInputResult::Complete(line)
            }
            // Printable ASCII character
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
