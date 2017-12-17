/// Represents a series of offsets within a text that represent the starting position
/// of each line.
pub struct LineMap {
    line_breaks: Vec<u64>,
}

impl LineMap {
    /// Creates a `LineMap` from the provided list of line breaks.
    pub fn new<V: Into<Vec<u64>>>(line_breaks: V) -> LineMap {
        LineMap {
            line_breaks: line_breaks.into(),
        }
    }

    /// Parses the provided string to find the line breaks and builds a `LineMap` from the result.
    pub fn parse(text: &str) -> LineMap {
        let mut last_char = '\0';
        let mut last_idx = 0;
        let mut line_breaks = Vec::new();
        for (idx, c) in text.char_indices() {
            if last_char == '\r' && c != '\n' {
                // Last was CR, but this isn't LF. So add the last char as a line-break
                line_breaks.push(last_idx as u64);
            }

            if c == '\n' {
                line_breaks.push(idx as u64);
            }

            last_char = c;
            last_idx = idx;
        }

        if last_char == '\r' {
            line_breaks.push(last_idx as u64);
        }

        LineMap::new(line_breaks)
    }

    /// Gets the list of line breaks.
    pub fn line_breaks(&self) -> &[u64] {
        &self.line_breaks
    }

    /// Maps the specified offset to a (`line`, `column`) pair.
    ///
    /// `line` is the 0-based line offset of the character.
    /// `offset` is the 0-based column offset of the character within the line.
    pub fn map_offset(&self, offset: u64) -> (u64, u64) {
        let line = match self.line_breaks.binary_search(&offset) {
            // This IS the line-break character
            // Thus idx is the index of this line break
            // We consider the line break to be the last character of the previous line
            Ok(idx) => idx,

            // This isn't the line-break character,
            // but binary_search gave us the offset where this value COULD be inserted into the
            // list, which is the offset of the line_break PRIOR to this character, and thus the line number
            Err(idx) => idx,
        };

        let col = if line == 0 {
            offset
        } else {
            offset - self.line_breaks[line - 1] - 1
        };

        (line as u64, col)
    }
}

#[cfg(test)]
mod tests {
    use super::LineMap;

    #[test]
    pub fn cr_is_a_line_break() {
        assert_eq!(&[1], LineMap::parse("a\rb").line_breaks());
    }

    #[test]
    pub fn lf_is_a_line_break() {
        assert_eq!(&[1], LineMap::parse("a\nb").line_breaks());
    }

    #[test]
    pub fn crlf_is_a_line_break() {
        assert_eq!(&[2], LineMap::parse("a\r\nb").line_breaks());
    }

    #[test]
    pub fn multi_line_complex_sequence() {
        assert_eq!(
            &[1, 3, 6, 8, 10, 11, 12],
            LineMap::parse("a\rb\nc\r\nd\r\r\n\n\r").line_breaks()
        )
    }

    #[test]
    pub fn map_offset_returns_correct_positions() {
        //                        01 23 45 6 78 9 0 1 2
        let map = LineMap::parse("a\rb\nc\r\nd\r\r\n\n\r");

        assert_eq!((0, 0), map.map_offset(0)); // "a"
        assert_eq!((0, 1), map.map_offset(1)); // "\r" LINE
        assert_eq!((1, 0), map.map_offset(2)); // "b"
        assert_eq!((1, 1), map.map_offset(3)); // "\n" LINE
        assert_eq!((2, 0), map.map_offset(4)); // "c"
        assert_eq!((2, 1), map.map_offset(5)); // "\r"
        assert_eq!((2, 2), map.map_offset(6)); // "\n" LINE
        assert_eq!((3, 0), map.map_offset(7)); // "d"
        assert_eq!((3, 1), map.map_offset(8)); // "\r" LINE
        assert_eq!((4, 0), map.map_offset(9)); // "\r"
        assert_eq!((4, 1), map.map_offset(10)); // "\n" LINE
        assert_eq!((5, 0), map.map_offset(11)); // "\n" LINE
        assert_eq!((6, 0), map.map_offset(12)); // "\r" LINE
    }
}
