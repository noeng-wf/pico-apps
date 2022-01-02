//! Helper function to split a command line into arguments:
//! - Skips whitespace (gap) in between
//! - Supports single and double quotes to include whitespace into a token
//! - Supports escaping with backslash (next character is handled like a literal argument character and not a quote or gap)

use core::str;

#[derive(Debug, PartialEq)]
pub enum ArgumentError {
    ExceedsBuffer,
    UnterminatedEscape,
    UnterminatedQuote(char),
}

enum ArgumentChar {
    Gap,
    Content(char)
}

struct ArgumentCharFsm {
    escaped: bool,
    in_quote: Option<char>,
}

impl ArgumentCharFsm {
    fn new() -> Self {
        Self {
            escaped: false,
            in_quote: None,
        }
    }

    fn feed_char(&mut self, c: char) -> Option<ArgumentChar> {
        if self.escaped {
            // Escaped character
            self.escaped = false;
            Some(ArgumentChar::Content(c))
        } else {
            if c == '\\' {
                // Backslash = escape
                self.escaped = true;
                None
            } else if let Some(quote) = self.in_quote {
                // In quotes
                if quote == c {
                    // Quote ends
                    self.in_quote = None;
                    None
                } else {
                    // Any other character (even whitespace)
                    Some(ArgumentChar::Content(c))
                }
            } else {
                if c == '\'' || c == '\"' {
                    // Quote starts
                    self.in_quote = Some(c);
                    None
                } else if c.is_ascii_whitespace() {
                    // Insignificant whitespace = gap
                    Some(ArgumentChar::Gap)
                } else {
                    // Any other character
                    Some(ArgumentChar::Content(c))
                }
            }
        }
    }

    fn complete(self) -> Result<(), ArgumentError> {
        if self.escaped {
            Err(ArgumentError::UnterminatedEscape)
        } else if let Some(c) = self.in_quote {
            Err(ArgumentError::UnterminatedQuote(c))
        } else {
            Ok(())
        }
    }
}

pub fn split_first_argument<'a, 'b>(source: &'a str, buffer: &'b mut [u8]) -> Result<(&'b str, &'a str), ArgumentError> {
    let mut fsm = ArgumentCharFsm::new();
    let mut buffer_offset: usize = 0;
    let mut next_argument_offset = source.len(); // assume first that the argument is the whole source string
    for (source_offset, c) in source.char_indices() {
        if let Some(arg_char) = fsm.feed_char(c) {
            match arg_char {
                ArgumentChar::Content(c) => {
                    let len = c.len_utf8();
                    if buffer_offset + len <= buffer.len() {
                        c.encode_utf8(&mut buffer[buffer_offset..buffer_offset+len]);
                        buffer_offset += len;
                    } else {
                        return Err(ArgumentError::ExceedsBuffer);
                    }
                },
                ArgumentChar::Gap => {
                    if buffer_offset > 0 {
                        // End of argument found
                        next_argument_offset = source_offset; // argument is shorter than the whole string 
                        break;
                    } else {
                        // Still in the gap before the argument
                    }
                }
            }
        }
    }
    fsm.complete()?;

    let first_arg = str::from_utf8(&buffer[..buffer_offset]).unwrap(); // should always be valid UTF-8 because the source was of type str
    let remaining_source_str = &source[next_argument_offset..];
    Ok((first_arg, remaining_source_str))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let mut buffer = vec![0;10];
        let (first_arg, remaining_source_str) = split_first_argument("", &mut buffer).unwrap();
        assert_eq!(first_arg, "");
        assert_eq!(remaining_source_str, "");
    }

    #[test]
    fn test_single_arg() {
        let mut buffer = vec![0;10];
        let (first_arg, remaining_source_str) = split_first_argument("abc", &mut buffer).unwrap();
        assert_eq!(first_arg, "abc");
        assert_eq!(remaining_source_str, "");
    }

    #[test]
    fn test_single_arg_leading_trailing_space() {
        let mut buffer = vec![0;10];
        let (first_arg, remaining_source_str) = split_first_argument("\n abc \t", &mut buffer).unwrap();
        assert_eq!(first_arg, "abc");
        assert_eq!(remaining_source_str, " \t");
    }

    #[test]
    fn test_multiple_args() {
        let mut buffer = vec![0;20];
        let (first_arg, remaining_source_str) = split_first_argument("abc 123 $", &mut buffer).unwrap();
        assert_eq!(first_arg, "abc");
        assert_eq!(remaining_source_str, " 123 $");
    }

    #[test]
    fn test_full_buffer() {
        let mut buffer = vec![0;6];
        let (first_arg, remaining_source_str) = split_first_argument("abcdef", &mut buffer).unwrap();
        assert_eq!(first_arg, "abcdef");
        assert_eq!(remaining_source_str, "");
    }

    #[test]
    fn test_full_buffer_plus_1() {
        let mut buffer = vec![0;6];
        let err = split_first_argument("abcdefg", &mut buffer).unwrap_err();
        assert_eq!(err, ArgumentError::ExceedsBuffer);
    }

    #[test]
    fn test_single_quote() {
        let mut buffer = vec![0;20];
        let (first_arg, remaining_source_str) = split_first_argument("'abc\"def   '   ghi", &mut buffer).unwrap();
        assert_eq!(first_arg, "abc\"def   ");
        assert_eq!(remaining_source_str, "   ghi");
    }

    #[test]
    fn test_double_quote() {
        let mut buffer = vec![0;20];
        let (first_arg, remaining_source_str) = split_first_argument(" \" hello '123\" abc", &mut buffer).unwrap();
        assert_eq!(first_arg, " hello '123");
        assert_eq!(remaining_source_str, " abc");
    }

    #[test]
    fn test_escape_space() {
        let mut buffer = vec![0;20];
        let (first_arg, remaining_source_str) = split_first_argument(" \\ \\  hello", &mut buffer).unwrap();
        assert_eq!(first_arg, "  ");
        assert_eq!(remaining_source_str, " hello");
    }

    #[test]
    fn test_escape_any() {
        let mut buffer = vec![0;20];
        let (first_arg, remaining_source_str) = split_first_argument("ab\\cde\\f\\  hello", &mut buffer).unwrap();
        assert_eq!(first_arg, "abcdef ");
        assert_eq!(remaining_source_str, " hello");
    }

    #[test]
    fn test_escape_quote_outside() {
        let mut buffer = vec![0;20];
        let (first_arg, remaining_source_str) = split_first_argument(" \\\"abc \\\"def", &mut buffer).unwrap();
        assert_eq!(first_arg, "\"abc");
        assert_eq!(remaining_source_str, " \\\"def");
    }

    #[test]
    fn test_escape_quote_inside() {
        let mut buffer = vec![0;20];
        let (first_arg, remaining_source_str) = split_first_argument("\"abc \\\" def\" ghi", &mut buffer).unwrap();
        assert_eq!(first_arg, "abc \" def");
        assert_eq!(remaining_source_str, " ghi");
    }

    #[test]
    fn test_concatenate_quoted_parts() {
        let mut buffer = vec![0;20];
        let (first_arg, remaining_source_str) = split_first_argument("abc'de\"fg'\"h'i j\"klm 123", &mut buffer).unwrap();
        assert_eq!(first_arg, "abcde\"fgh'i jklm");
        assert_eq!(remaining_source_str, " 123");
    }

    #[test]
    fn test_unterminated_single_quote() {
        let mut buffer = vec![0;10];
        let err = split_first_argument("abc'def ", &mut buffer).unwrap_err();
        assert_eq!(err, ArgumentError::UnterminatedQuote('\''));
    }

    #[test]
    fn test_unterminated_double_quote() {
        let mut buffer = vec![0;10];
        let err = split_first_argument("abc\"def ", &mut buffer).unwrap_err();
        assert_eq!(err, ArgumentError::UnterminatedQuote('\"'));
    }

    #[test]
    fn test_unterminated_escape() {
        let mut buffer = vec![0;10];
        let err = split_first_argument("abc\\", &mut buffer).unwrap_err();
        assert_eq!(err, ArgumentError::UnterminatedEscape);
    }
}
