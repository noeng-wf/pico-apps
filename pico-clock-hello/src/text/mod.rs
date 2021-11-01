mod font;

use font::{DEFAULT_FONT, FONT_HEIGHT};

pub const TEXT_BITMAP_HEIGHT: usize = FONT_HEIGHT;
const CHARACTER_GAP: usize = 1;

#[derive(Debug)]
pub enum TextRenderError {
    TextTooLong,
    UnsupportedCharacter,
}

pub struct TextBitmap {
    pub width: usize,
    pub data: [u128; TEXT_BITMAP_HEIGHT],
}

impl TextBitmap {
    pub fn from_str(text: &str) -> Result<Self, TextRenderError> {
        let mut text_bitmap = Self {
            width: 0,
            data: [0; TEXT_BITMAP_HEIGHT],
        };
        text_bitmap.append_text(text)?;

        Ok(text_bitmap)
    }

    pub fn append_text(&mut self, text: &str) -> Result<(), TextRenderError> {
        for c in text.chars() {
            if let Some(x) = DEFAULT_FONT.get_character(c) {
                self.append_character(x)?;
            } else {
                return Err(TextRenderError::UnsupportedCharacter);
            }
        }
        Ok(())
    }

    /// Extracts a horizontal segment from an instance.
    ///
    /// The resulting segment always has the requested width. Those parts that are out of the
    /// instance bounds are filled with cleared bits (useful to implement moving section for
    /// scrolling).
    pub fn segment(&self, offset: isize, width: usize) -> Self {
        assert!(width <= 128);

        let mut result = Self {
            width,
            data: [0; TEXT_BITMAP_HEIGHT],
        };

        let mask = if width < 128 {
            (1 << width) - 1
        } else {
            u128::MAX
        };
        for row in 0..TEXT_BITMAP_HEIGHT {
            result.data[row] = match offset {
                -127..=-1 => (self.data[row] << -offset) & mask,
                0 => self.data[row] & mask,
                1..=127 => (self.data[row] >> offset) & mask,
                _ => 0,
            };
        }

        result
    }

    fn append_character(&mut self, character: &font::FontCharacter) -> Result<(), TextRenderError> {
        if self.width + CHARACTER_GAP + (character.width as usize) > 128 {
            return Err(TextRenderError::TextTooLong);
        }

        for row in 0..TEXT_BITMAP_HEIGHT {
            self.data[row] |= (character.bit_pattern[row] as u128) << (self.width + CHARACTER_GAP);
        }
        self.width += CHARACTER_GAP + (character.width as usize);

        Ok(())
    }
}
