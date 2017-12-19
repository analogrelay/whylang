use std::char;
use std::io::Read;

use utils;

use text::{TextSpan, Error};

/// Represents a sliding window of text.
pub struct TextWindow<'a> {
    buf: &'a str,
    offset: usize,
    end: usize,
    last: Option<char>,
}

impl<'a> TextWindow<'a> {
    pub fn new(buf: &'a str) -> TextWindow<'a> {
        TextWindow {
            buf,
            offset: 0,
            end: 0,
            last: None,
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn end(&self) -> usize {
        self.end
    }

    /// Gets the last item in the window as a `char`
    /// 
    /// This is useful as it is usually the char callers are most interested in.
    pub fn last(&self) -> Option<char> {
        self.last
    }

    pub fn span(&self) -> TextSpan {
        TextSpan::new(self.offset, self.end)
    }

    /// Gets a `str` that represents the current content of the window
    pub fn as_str(&self) -> &str {
        &self.buf[self.offset..self.end]
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buf.as_bytes()[self.offset..self.end]
    }

    /// Load another character into the buffer
    ///
    /// ## Returns
    /// `true` if a character is successfully read in.
    /// `false` if end-of-file has been reached.
    pub fn next(&mut self) -> Result<bool, Error> {
        if self.end >= self.buf.len() {
            self.last = None;
            Ok(false)
        } else {
            // See how many bytes are needed to load the character pointed to by `end`
            let width = utils::utf8_char_width(self.buf.as_bytes()[self.end]);

            // Advance that number of bytes
            let new_end = self.end + width;

            if let Some((c, _)) = utils::decode_utf8_character(&self.buf.as_bytes()[self.end..new_end]) {
                self.last = Some(c);
                self.end = new_end;
                Ok(true)
            } else {
                // TODO: Use Result?
                Err(Error::InvalidText)
            }
        }
    }

    /// Reset the end point of the window to the provided offset
    ///
    /// # Panics
    /// Panics if the new offset is not a character boundary. To avoid this, only use values received from `.end()` as
    /// inputs to this method.
    pub fn backtrack(&mut self, new_end: usize) {
        assert!(
            self.buf.is_char_boundary(new_end),
            "The requested index does not represent a character boundary.");
        self.end = new_end;
    }

    /// Advances the window to the point currently pointed to by `end`
    /// 
    /// After this call, `.as_str()` will return an empty string until the next call
    /// to `next`
    pub fn advance(&mut self) {
        self.offset = self.end;
        self.last = None;
    }
}

#[cfg(test)]
mod tests {
    use text::TextWindow;

    #[test]
    pub fn as_str_returns_empty_string_when_window_initialized() {
        assert_eq!("", TextWindow::new("testwin").as_str());
    }

    #[test]
    pub fn as_bytes_returns_empty_string_when_window_initialized() {
        assert_eq!(b"", TextWindow::new("testwin").as_bytes());
    }

    #[test]
    pub fn last_returns_none_when_window_initialized() {
        assert_eq!(None, TextWindow::new("testwin").last());
    }

    #[test]
    pub fn next_loads_next_character_into_buffer() {
        let mut window = TextWindow::new("testwin");
        assert!(window.next().unwrap());
        assert_eq!("t", window.as_str());
    }

    #[test]
    pub fn next_returns_false_when_at_end_of_file() {
        let mut window = TextWindow::new("testwin");
        for _ in 0..7 {
            assert!(window.next().unwrap());
        }
        assert!(!window.next().unwrap());
        assert_eq!("testwin", window.as_str());
    }

    #[test]
    pub fn backtrack_moves_end_pointer_back_to_provided_value() {
        let mut window = TextWindow::new("testwin");
        assert!(window.next().unwrap());
        assert!(window.next().unwrap());
        assert!(window.next().unwrap());
        assert!(window.next().unwrap());
        assert_eq!("test", window.as_str());
        assert_eq!(b"test", window.as_bytes());
        let marker = window.end();
        assert!(window.next().unwrap());
        assert!(window.next().unwrap());
        assert!(window.next().unwrap());
        assert_eq!("testwin", window.as_str());
        window.backtrack(marker);
        assert_eq!("test", window.as_str());
    }

    #[test]
    pub fn advance_moves_offset_up_to_end_pointer() {
        let mut window = TextWindow::new("testwin");
        assert!(window.next().unwrap());
        assert!(window.next().unwrap());
        assert!(window.next().unwrap());
        assert!(window.next().unwrap());
        assert_eq!("test", window.as_str());
        window.advance();
        assert_eq!("", window.as_str());
        assert_eq!(None, window.last());
        assert!(window.next().unwrap());
        assert!(window.next().unwrap());
        assert!(window.next().unwrap());
        assert_eq!("win", window.as_str());
    }

    #[test]
    pub fn next_moves_in_character_increments() {
        let mut window = TextWindow::new("a¶Ё₵𐆓e\u{0301}");
        move_next_and_check(&mut window, "a", 'a', 1);
        move_next_and_check(&mut window, "a¶", '¶', 3);
        move_next_and_check(&mut window, "a¶Ё", 'Ё', 5);
        move_next_and_check(&mut window, "a¶Ё₵", '₵', 8);
        move_next_and_check(&mut window, "a¶Ё₵𐆓", '𐆓', 12);
        move_next_and_check(&mut window, "a¶Ё₵𐆓e", 'e', 13);
        move_next_and_check(&mut window, "a¶Ё₵𐆓e\u{0301}", '\u{0301}', 15);
    }

    fn move_next_and_check(window: &mut TextWindow, expected_str: &'static str, expected_last: char, expected_index: usize) {
        assert!(window.next().unwrap());
        assert_eq!(expected_str, window.as_str());
        assert_eq!(Some(expected_last), window.last());
        assert_eq!(expected_index, window.end);
    }
}
