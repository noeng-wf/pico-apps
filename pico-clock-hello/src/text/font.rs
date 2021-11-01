//! Definition of dot patterns for all printable ASCII characters, having a height of 7 dots
//! and most of them have a width of 4 dots.
//! The numeric digits all have a constant width of 4 dots to simplify displaying time.

pub const FONT_HEIGHT: usize = 7;

/// Dot pattern of a particular character.
pub struct FontCharacter {
    /// Width in bits (starting at LSB).
    pub width: u8,
    /// Dot pattern coded as a bit mask (LSB is the leftmost dot), as array of rows.
    pub bit_pattern: [u8; FONT_HEIGHT],
}

const FIRST_FONT_CHARACTER: char = ' ';
const LAST_FONT_CHARACTER: char = '~';
const NUMBER_OF_FONT_CHARACTERS: usize =
    (LAST_FONT_CHARACTER as usize) - (FIRST_FONT_CHARACTER as usize) + 1;

pub struct Font {
    characters: [FontCharacter; NUMBER_OF_FONT_CHARACTERS],
}

impl Font {
    /// Gets the dot pattern for the given character. Only printable ASCII characters are available.
    pub fn get_character(&self, c: char) -> Option<&FontCharacter> {
        if c >= FIRST_FONT_CHARACTER && c <= LAST_FONT_CHARACTER {
            Some(&self.characters[(c as usize) - (FIRST_FONT_CHARACTER as usize)])
        } else {
            None
        }
    }
}

pub const DEFAULT_FONT: Font = Font {
    characters: [
        // Character ' '
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
            ],
        },
        // Character '!'
        FontCharacter {
            width: 1u8,
            bit_pattern: [
                reverse_bits(0b1, 1),
                reverse_bits(0b1, 1),
                reverse_bits(0b1, 1),
                reverse_bits(0b1, 1),
                reverse_bits(0b1, 1),
                reverse_bits(0b0, 1),
                reverse_bits(0b1, 1),
            ],
        },
        // Character '"'
        FontCharacter {
            width: 3u8,
            bit_pattern: [
                reverse_bits(0b101, 3),
                reverse_bits(0b101, 3),
                reverse_bits(0b000, 3),
                reverse_bits(0b000, 3),
                reverse_bits(0b000, 3),
                reverse_bits(0b000, 3),
                reverse_bits(0b000, 3),
            ],
        },
        // Character '#'
        FontCharacter {
            width: 5u8,
            bit_pattern: [
                reverse_bits(0b00000, 5),
                reverse_bits(0b01010, 5),
                reverse_bits(0b11111, 5),
                reverse_bits(0b01010, 5),
                reverse_bits(0b01010, 5),
                reverse_bits(0b11111, 5),
                reverse_bits(0b01010, 5),
            ],
        },
        // Character '$'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0010, 4),
                reverse_bits(0b0111, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b0110, 4),
                reverse_bits(0b0001, 4),
                reverse_bits(0b1110, 4),
                reverse_bits(0b0010, 4),
            ],
        },
        // Character '%'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b1100, 4),
                reverse_bits(0b1101, 4),
                reverse_bits(0b0001, 4),
                reverse_bits(0b0110, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1011, 4),
                reverse_bits(0b0011, 4),
            ],
        },
        // Character '&'
        FontCharacter {
            width: 5u8,
            bit_pattern: [
                reverse_bits(0b00100, 5),
                reverse_bits(0b01010, 5),
                reverse_bits(0b00100, 5),
                reverse_bits(0b01101, 5),
                reverse_bits(0b10010, 5),
                reverse_bits(0b10010, 5),
                reverse_bits(0b01101, 5),
            ],
        },
        // Character '''
        FontCharacter {
            width: 1u8,
            bit_pattern: [
                reverse_bits(0b1, 1),
                reverse_bits(0b1, 1),
                reverse_bits(0b0, 1),
                reverse_bits(0b0, 1),
                reverse_bits(0b0, 1),
                reverse_bits(0b0, 1),
                reverse_bits(0b0, 1),
            ],
        },
        // Character '('
        FontCharacter {
            width: 3u8,
            bit_pattern: [
                reverse_bits(0b001, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b100, 3),
                reverse_bits(0b100, 3),
                reverse_bits(0b100, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b001, 3),
            ],
        },
        // Character '),'
        FontCharacter {
            width: 3u8,
            bit_pattern: [
                reverse_bits(0b100, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b001, 3),
                reverse_bits(0b001, 3),
                reverse_bits(0b001, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b100, 3),
            ],
        },
        // Character '*'
        FontCharacter {
            width: 5u8,
            bit_pattern: [
                reverse_bits(0b00000, 5),
                reverse_bits(0b10101, 5),
                reverse_bits(0b01110, 5),
                reverse_bits(0b11111, 5),
                reverse_bits(0b01110, 5),
                reverse_bits(0b10101, 5),
                reverse_bits(0b00000, 5),
            ],
        },
        // Character '+'
        FontCharacter {
            width: 5u8,
            bit_pattern: [
                reverse_bits(0b00000, 5),
                reverse_bits(0b00100, 5),
                reverse_bits(0b00100, 5),
                reverse_bits(0b11111, 5),
                reverse_bits(0b00100, 5),
                reverse_bits(0b00100, 5),
                reverse_bits(0b00000, 5),
            ],
        },
        // Character ','
        FontCharacter {
            width: 2u8,
            bit_pattern: [
                reverse_bits(0b00, 2),
                reverse_bits(0b00, 2),
                reverse_bits(0b00, 2),
                reverse_bits(0b11, 2),
                reverse_bits(0b11, 2),
                reverse_bits(0b01, 2),
                reverse_bits(0b10, 2),
            ],
        },
        // Character '-'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b1111, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
            ],
        },
        // Character '.'
        FontCharacter {
            width: 2u8,
            bit_pattern: [
                reverse_bits(0b00, 2),
                reverse_bits(0b00, 2),
                reverse_bits(0b00, 2),
                reverse_bits(0b00, 2),
                reverse_bits(0b00, 2),
                reverse_bits(0b11, 2),
                reverse_bits(0b11, 2),
            ],
        },
        // Character '/'
        FontCharacter {
            width: 3u8,
            bit_pattern: [
                reverse_bits(0b001, 3),
                reverse_bits(0b001, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b100, 3),
                reverse_bits(0b100, 3),
            ],
        },
        // Character '0'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b0110, 4),
            ],
        },
        // Character '1'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0010, 4),
                reverse_bits(0b0110, 4),
                reverse_bits(0b0010, 4),
                reverse_bits(0b0010, 4),
                reverse_bits(0b0010, 4),
                reverse_bits(0b0010, 4),
                reverse_bits(0b0111, 4),
            ],
        },
        // Character '2'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b0001, 4),
                reverse_bits(0b0010, 4),
                reverse_bits(0b0100, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1111, 4),
            ],
        },
        // Character '3'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b0001, 4),
                reverse_bits(0b0110, 4),
                reverse_bits(0b0001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b0110, 4),
            ],
        },
        // Character '4'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0001, 4),
                reverse_bits(0b0011, 4),
                reverse_bits(0b0101, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1111, 4),
                reverse_bits(0b0001, 4),
                reverse_bits(0b0001, 4),
            ],
        },
        // Character '5'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b1111, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1110, 4),
                reverse_bits(0b0001, 4),
                reverse_bits(0b0001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b0110, 4),
            ],
        },
        // Character '6'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0010, 4),
                reverse_bits(0b0100, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b0110, 4),
            ],
        },
        // Character '7'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b1111, 4),
                reverse_bits(0b0001, 4),
                reverse_bits(0b0001, 4),
                reverse_bits(0b0010, 4),
                reverse_bits(0b0100, 4),
                reverse_bits(0b0100, 4),
                reverse_bits(0b0100, 4),
            ],
        },
        // Character '8'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b0110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b0110, 4),
            ],
        },
        // Character '9'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b0111, 4),
                reverse_bits(0b0001, 4),
                reverse_bits(0b0010, 4),
                reverse_bits(0b0100, 4),
            ],
        },
        // Character ':'
        FontCharacter {
            width: 2u8,
            bit_pattern: [
                reverse_bits(0b00, 2),
                reverse_bits(0b11, 2),
                reverse_bits(0b11, 2),
                reverse_bits(0b00, 2),
                reverse_bits(0b11, 2),
                reverse_bits(0b11, 2),
                reverse_bits(0b00, 2),
            ],
        },
        // Character ';'
        FontCharacter {
            width: 2u8,
            bit_pattern: [
                reverse_bits(0b00, 2),
                reverse_bits(0b11, 2),
                reverse_bits(0b11, 2),
                reverse_bits(0b00, 2),
                reverse_bits(0b11, 2),
                reverse_bits(0b01, 2),
                reverse_bits(0b10, 2),
            ],
        },
        // Character '<'
        FontCharacter {
            width: 3u8,
            bit_pattern: [
                reverse_bits(0b000, 3),
                reverse_bits(0b001, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b100, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b001, 3),
                reverse_bits(0b000, 3),
            ],
        },
        // Character '='
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b1111, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b1111, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
            ],
        },
        // Character '>'
        FontCharacter {
            width: 3u8,
            bit_pattern: [
                reverse_bits(0b000, 3),
                reverse_bits(0b100, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b001, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b100, 3),
                reverse_bits(0b000, 3),
            ],
        },
        // Character '?'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b0001, 4),
                reverse_bits(0b0010, 4),
                reverse_bits(0b0100, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b0100, 4),
            ],
        },
        // Character '@'
        FontCharacter {
            width: 5u8,
            bit_pattern: [
                reverse_bits(0b01110, 5),
                reverse_bits(0b10001, 5),
                reverse_bits(0b10111, 5),
                reverse_bits(0b10101, 5),
                reverse_bits(0b10111, 5),
                reverse_bits(0b10000, 5),
                reverse_bits(0b01110, 5),
            ],
        },
        // Character 'A'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1111, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
            ],
        },
        // Character 'B'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b1110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1110, 4),
            ],
        },
        // Character 'C'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b0110, 4),
            ],
        },
        // Character 'D'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b1100, 4),
                reverse_bits(0b1010, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1010, 4),
                reverse_bits(0b1100, 4),
            ],
        },
        // Character 'E'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b1111, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1110, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1111, 4),
            ],
        },
        // Character 'F'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b1111, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1110, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1000, 4),
            ],
        },
        // Character 'G'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1011, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b0111, 4),
            ],
        },
        // Character 'H'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1111, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
            ],
        },
        // Character 'I'
        FontCharacter {
            width: 3u8,
            bit_pattern: [
                reverse_bits(0b111, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b111, 3),
            ],
        },
        // Character 'J'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0111, 4),
                reverse_bits(0b0010, 4),
                reverse_bits(0b0010, 4),
                reverse_bits(0b0010, 4),
                reverse_bits(0b1010, 4),
                reverse_bits(0b1010, 4),
                reverse_bits(0b0100, 4),
            ],
        },
        // Character 'K'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b1001, 4),
                reverse_bits(0b1010, 4),
                reverse_bits(0b1100, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1100, 4),
                reverse_bits(0b1010, 4),
                reverse_bits(0b1001, 4),
            ],
        },
        // Character 'L'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b1000, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1111, 4),
            ],
        },
        // Character 'M'
        FontCharacter {
            width: 5u8,
            bit_pattern: [
                reverse_bits(0b10001, 5),
                reverse_bits(0b11011, 5),
                reverse_bits(0b10101, 5),
                reverse_bits(0b10101, 5),
                reverse_bits(0b10001, 5),
                reverse_bits(0b10001, 5),
                reverse_bits(0b10001, 5),
            ],
        },
        // Character 'N'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1101, 4),
                reverse_bits(0b1011, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
            ],
        },
        // Character 'O'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b0110, 4),
            ],
        },
        // Character 'P'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b1110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1110, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1000, 4),
            ],
        },
        // Character 'Q'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1101, 4),
                reverse_bits(0b0110, 4),
                reverse_bits(0b0001, 4),
            ],
        },
        // Character 'R'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b1110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1110, 4),
                reverse_bits(0b1100, 4),
                reverse_bits(0b1010, 4),
                reverse_bits(0b1001, 4),
            ],
        },
        // Character 'S'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b0110, 4),
                reverse_bits(0b0001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b0110, 4),
            ],
        },
        // Character 'T'
        FontCharacter {
            width: 3u8,
            bit_pattern: [
                reverse_bits(0b111, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
            ],
        },
        // Character 'U'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b0110, 4),
            ],
        },
        // Character 'V'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1010, 4),
                reverse_bits(0b1100, 4),
            ],
        },
        // Character 'W'
        FontCharacter {
            width: 5u8,
            bit_pattern: [
                reverse_bits(0b10001, 5),
                reverse_bits(0b10001, 5),
                reverse_bits(0b10001, 5),
                reverse_bits(0b10101, 5),
                reverse_bits(0b10101, 5),
                reverse_bits(0b10101, 5),
                reverse_bits(0b01010, 5),
            ],
        },
        // Character 'X'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b0110, 4),
                reverse_bits(0b0110, 4),
                reverse_bits(0b0110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
            ],
        },
        // Character 'Y'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b0111, 4),
                reverse_bits(0b0001, 4),
                reverse_bits(0b0001, 4),
                reverse_bits(0b0001, 4),
            ],
        },
        // Character 'Z'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b1111, 4),
                reverse_bits(0b0001, 4),
                reverse_bits(0b0010, 4),
                reverse_bits(0b0100, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1111, 4),
            ],
        },
        // Character '['
        FontCharacter {
            width: 2u8,
            bit_pattern: [
                reverse_bits(0b11, 2),
                reverse_bits(0b10, 2),
                reverse_bits(0b10, 2),
                reverse_bits(0b10, 2),
                reverse_bits(0b10, 2),
                reverse_bits(0b10, 2),
                reverse_bits(0b11, 2),
            ],
        },
        // Character '\'
        FontCharacter {
            width: 3u8,
            bit_pattern: [
                reverse_bits(0b100, 3),
                reverse_bits(0b100, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b001, 3),
                reverse_bits(0b001, 3),
            ],
        },
        // Character ']'
        FontCharacter {
            width: 2u8,
            bit_pattern: [
                reverse_bits(0b11, 2),
                reverse_bits(0b01, 2),
                reverse_bits(0b01, 2),
                reverse_bits(0b01, 2),
                reverse_bits(0b01, 2),
                reverse_bits(0b01, 2),
                reverse_bits(0b11, 2),
            ],
        },
        // Character '^'
        FontCharacter {
            width: 3u8,
            bit_pattern: [
                reverse_bits(0b010, 3),
                reverse_bits(0b101, 3),
                reverse_bits(0b000, 3),
                reverse_bits(0b000, 3),
                reverse_bits(0b000, 3),
                reverse_bits(0b000, 3),
                reverse_bits(0b000, 3),
            ],
        },
        // Character '_'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b1111, 4),
            ],
        },
        // Character '`'
        FontCharacter {
            width: 2u8,
            bit_pattern: [
                reverse_bits(0b10, 2),
                reverse_bits(0b01, 2),
                reverse_bits(0b00, 2),
                reverse_bits(0b00, 2),
                reverse_bits(0b00, 2),
                reverse_bits(0b00, 2),
                reverse_bits(0b00, 2),
            ],
        },
        // Character 'a'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b0110, 4),
                reverse_bits(0b0001, 4),
                reverse_bits(0b0111, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b0111, 4),
            ],
        },
        // Character 'b'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b1000, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1110, 4),
            ],
        },
        // Character 'c'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b0110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b0111, 4),
            ],
        },
        // Character 'd'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0001, 4),
                reverse_bits(0b0001, 4),
                reverse_bits(0b0111, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b0111, 4),
            ],
        },
        // Character 'e'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b0110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1111, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b0111, 4),
            ],
        },
        // Character 'f'
        FontCharacter {
            width: 3u8,
            bit_pattern: [
                reverse_bits(0b001, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b111, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
            ],
        },
        // Character 'g'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0000, 4),
                reverse_bits(0b0110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b0111, 4),
                reverse_bits(0b0001, 4),
                reverse_bits(0b1110, 4),
            ],
        },
        // Character 'h'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b1000, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
            ],
        },
        // Character 'i'
        FontCharacter {
            width: 3u8,
            bit_pattern: [
                reverse_bits(0b010, 3),
                reverse_bits(0b000, 3),
                reverse_bits(0b110, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b111, 3),
            ],
        },
        // Character 'j'
        FontCharacter {
            width: 3u8,
            bit_pattern: [
                reverse_bits(0b001, 3),
                reverse_bits(0b000, 3),
                reverse_bits(0b011, 3),
                reverse_bits(0b001, 3),
                reverse_bits(0b001, 3),
                reverse_bits(0b101, 3),
                reverse_bits(0b010, 3),
            ],
        },
        // Character 'k'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b1000, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1010, 4),
                reverse_bits(0b1100, 4),
                reverse_bits(0b1010, 4),
                reverse_bits(0b1001, 4),
            ],
        },
        // Character 'l'
        FontCharacter {
            width: 3u8,
            bit_pattern: [
                reverse_bits(0b110, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b111, 3),
            ],
        },
        // Character 'm'
        FontCharacter {
            width: 5u8,
            bit_pattern: [
                reverse_bits(0b00000, 5),
                reverse_bits(0b00000, 5),
                reverse_bits(0b11110, 5),
                reverse_bits(0b10101, 5),
                reverse_bits(0b10101, 5),
                reverse_bits(0b10101, 5),
                reverse_bits(0b10101, 5),
            ],
        },
        // Character 'n'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b1110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
            ],
        },
        // Character 'o'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b0110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b0110, 4),
            ],
        },
        // Character 'p'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b1110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1110, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1000, 4),
            ],
        },
        // Character 'q'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b0110, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b0111, 4),
                reverse_bits(0b0001, 4),
                reverse_bits(0b0001, 4),
            ],
        },
        // Character 'r'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b1010, 4),
                reverse_bits(0b1101, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b1000, 4),
            ],
        },
        // Character 's'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b0111, 4),
                reverse_bits(0b1000, 4),
                reverse_bits(0b0110, 4),
                reverse_bits(0b0001, 4),
                reverse_bits(0b1110, 4),
            ],
        },
        // Character 't'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0100, 4),
                reverse_bits(0b0100, 4),
                reverse_bits(0b1110, 4),
                reverse_bits(0b0100, 4),
                reverse_bits(0b0100, 4),
                reverse_bits(0b0101, 4),
                reverse_bits(0b0010, 4),
            ],
        },
        // Character 'u'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b0111, 4),
            ],
        },
        // Character 'v'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1010, 4),
                reverse_bits(0b1100, 4),
            ],
        },
        // Character 'w'
        FontCharacter {
            width: 5u8,
            bit_pattern: [
                reverse_bits(0b00000, 5),
                reverse_bits(0b00000, 5),
                reverse_bits(0b10101, 5),
                reverse_bits(0b10101, 5),
                reverse_bits(0b10101, 5),
                reverse_bits(0b10101, 5),
                reverse_bits(0b01010, 5),
            ],
        },
        // Character 'x'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b0110, 4),
                reverse_bits(0b0110, 4),
                reverse_bits(0b0110, 4),
                reverse_bits(0b1001, 4),
            ],
        },
        // Character 'y'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b1001, 4),
                reverse_bits(0b0111, 4),
                reverse_bits(0b0001, 4),
                reverse_bits(0b1110, 4),
            ],
        },
        // Character 'z'
        FontCharacter {
            width: 4u8,
            bit_pattern: [
                reverse_bits(0b0000, 4),
                reverse_bits(0b0000, 4),
                reverse_bits(0b1111, 4),
                reverse_bits(0b0001, 4),
                reverse_bits(0b0010, 4),
                reverse_bits(0b0100, 4),
                reverse_bits(0b1111, 4),
            ],
        },
        // Character '{'
        FontCharacter {
            width: 3u8,
            bit_pattern: [
                reverse_bits(0b001, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b100, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b001, 3),
            ],
        },
        // Character '|'
        FontCharacter {
            width: 1u8,
            bit_pattern: [
                reverse_bits(0b1, 1),
                reverse_bits(0b1, 1),
                reverse_bits(0b1, 1),
                reverse_bits(0b1, 1),
                reverse_bits(0b1, 1),
                reverse_bits(0b1, 1),
                reverse_bits(0b1, 1),
            ],
        },
        // Character '}'
        FontCharacter {
            width: 3u8,
            bit_pattern: [
                reverse_bits(0b100, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b001, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b010, 3),
                reverse_bits(0b100, 3),
            ],
        },
        // Character '~'
        FontCharacter {
            width: 5u8,
            bit_pattern: [
                reverse_bits(0b00000, 5),
                reverse_bits(0b00000, 5),
                reverse_bits(0b01000, 5),
                reverse_bits(0b10101, 5),
                reverse_bits(0b00010, 5),
                reverse_bits(0b00000, 5),
                reverse_bits(0b00000, 5),
            ],
        },
    ],
};

/// Helper function that allows defining the bit pattern in a visually correct bit order above and
/// to reverse it afterwards (LSB is leftmost dot).
/// It is a const function that is evaluated at compile time and doesn't add any overhead at
/// runtime.
const fn reverse_bits(mut value: u32, mut len: usize) -> u8 {
    let mut new_value: u8 = 0;
    while len > 0 {
        new_value = (new_value << 1) | ((value & 1) as u8);
        value >>= 1;
        len -= 1;
    }
    new_value
}
