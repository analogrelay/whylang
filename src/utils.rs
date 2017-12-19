use std::char;

// utf8_char_width is nightly-only because it deals with string internals and I
// don't see a need to depend upon nightly just for that.
// So, this is a copy of that code from: https://doc.rust-lang.org/src/core/str/mod.rs.html#1553
// https://tools.ietf.org/html/rfc3629
static UTF8_CHAR_WIDTH: [u8; 256] = [
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x1F
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x3F
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x5F
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x7F
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0x9F
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0xBF
0,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,
2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2, // 0xDF
3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3, // 0xEF
4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0, // 0xFF
];

/// Given a first byte, determines how many bytes are in this UTF-8 character.
#[inline]
pub fn utf8_char_width(b: u8) -> usize {
    return UTF8_CHAR_WIDTH[b as usize] as usize;
}

// Another nightly thing brought local to use in stable: https://doc.rust-lang.org/src/core/char.rs.html#828
pub fn decode_utf8_character(buf: &[u8]) -> Option<(char, usize)> {
    // Emit InvalidSequence according to
    // Unicode §5.22 Best Practice for U+FFFD Substitution
    // http://www.unicode.org/versions/Unicode9.0.0/ch05.pdf#G40630

    // Roughly: consume at least one byte,
    // then validate one byte at a time and stop before the first unexpected byte
    // (which might be the valid start of the next byte sequence).

    let mut code_point;
    let mut next_idx = 1;
    macro_rules! first_byte {
        ($mask: expr) => {
            code_point = u32::from(buf[0] & $mask)
        }
    }
    macro_rules! continuation_byte {
        () => { continuation_byte!(0x80...0xBF) };
        ($range: pat) => {
            if next_idx > buf.len() {
                return None
            } else {
                match buf[next_idx] {
                    byte @ $range => {
                        code_point = (code_point << 6) | u32::from(byte & 0b0011_1111);
                        next_idx += 1;
                    }
                    _ => return None
                }
            }
        }
    }

    match buf[0] {
        0x00...0x7F => {
            first_byte!(0b1111_1111);
        }
        0xC2...0xDF => {
            first_byte!(0b0001_1111);
            continuation_byte!();
        }
        0xE0 => {
            first_byte!(0b0000_1111);
            continuation_byte!(0xA0...0xBF);  // 0x80...0x9F here are overlong
            continuation_byte!();
        }
        0xE1...0xEC | 0xEE...0xEF => {
            first_byte!(0b0000_1111);
            continuation_byte!();
            continuation_byte!();
        }
        0xED => {
            first_byte!(0b0000_1111);
            continuation_byte!(0x80...0x9F);  // 0xA0..0xBF here are surrogates
            continuation_byte!();
        }
        0xF0 => {
            first_byte!(0b0000_0111);
            continuation_byte!(0x90...0xBF);  // 0x80..0x8F here are overlong
            continuation_byte!();
            continuation_byte!();
        }
        0xF1...0xF3 => {
            first_byte!(0b0000_0111);
            continuation_byte!();
            continuation_byte!();
            continuation_byte!();
        }
        0xF4 => {
            first_byte!(0b0000_0111);
            continuation_byte!(0x80...0x8F);  // 0x90..0xBF here are beyond char::MAX
            continuation_byte!();
            continuation_byte!();
        }
        _ => return None  // Illegal first byte, overlong, or beyond MAX
    }
    unsafe {
        Some((char::from_u32_unchecked(code_point), next_idx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn utf8_char_width_returns_correct_width() {
        assert_eq!(1, utf8_char_width(0x24));
        assert_eq!(2, utf8_char_width(0xC2));
        assert_eq!(3, utf8_char_width(0xE2));
        assert_eq!(4, utf8_char_width(0xF0));
    }

    #[test]
    pub fn decode_utf8_character_decodes_character_correctly() {
        assert_eq!(Some(('$', 1)), decode_utf8_character(&[0x24]));
        assert_eq!(Some(('¢', 2)), decode_utf8_character(&[0xC2, 0xA2]));
        assert_eq!(Some(('€', 3)), decode_utf8_character(&[0xE2, 0x82, 0xAC]));
        assert_eq!(Some(('𐍈', 4)), decode_utf8_character(&[0xF0, 0x90, 0x8D, 0x88]));
    }
}