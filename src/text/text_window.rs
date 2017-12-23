use std::char;
use std::ops::{Range, RangeFrom, RangeFull, RangeTo, RangeInclusive, RangeToInclusive};

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
    pub fn take(&mut self) -> Result<bool, Error> {
        self.take_if(..)
    }

    /// Takes the next character if it meets the predicate
    ///
    /// ## Returns
    /// `true` if a character is successfully read in.
    /// `false` if the predicate did not match OR end-of-file has been reached.
    pub fn take_if<P: CharPredicate>(&mut self, predicate: P) -> Result<bool, Error> {
        if self.end >= self.buf.len() {
            self.last = None;
            Ok(false)
        } else {
            // See how many bytes are needed to load the character pointed to by `end`
            let width = utils::utf8_char_width(self.buf.as_bytes()[self.end]);

            // Advance that number of bytes
            let new_end = self.end + width;

            if let Some((c, _)) = utils::decode_utf8_character(&self.buf.as_bytes()[self.end..new_end]) {
                if predicate.test(c) {
                    self.last = Some(c);
                    self.end = new_end;
                    Ok(true)
                } else {
                    // Predicate doesn't match
                    Ok(false)
                }
            } else {
                Err(Error::InvalidText)
            }
        }
    }

    /// Resets the window back to the very beginning of the string.
    pub fn reset(&mut self) {
        self.offset = 0;
        self.end = 0;
        self.last = None;
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

        self.last = if self.end > 0 {
            // Update last

            // Scan back to the previous char boundary
            let mut pos = self.end - 1;
            while !self.buf.is_char_boundary(pos) {
                pos -= 1;
            }

            // Update last to that character
            let (c, _) = utils::decode_utf8_character(&self.buf.as_bytes()[pos..self.end])
                .expect("The previous character should have been a valid UTF-8 character.");
            Some(c)
        } else {
            None
        }
    }

    /// Advances the window to the point currently pointed to by `end`
    /// 
    /// After this call, `.as_str()` will return an empty string until the next call
    /// to `next`
    pub fn advance(&mut self) {
        self.offset = self.end;
        self.last = None;
    }

    /// Tests if the next character matches the provided predicate, without reading it into the buffer
    pub fn peek<P: CharPredicate>(&self, predicate: P) -> bool {
        if self.end >= self.buf.len() {
            return false;
        }

        if let Some((c, _)) = utils::decode_utf8_character(&self.buf.as_bytes()[self.end..]) {
            predicate.test(c)
        } else {
            false
        }
    }

    // Utilities for the tokenizer
    pub fn last_is<P: CharPredicate>(&self, predicate: P) -> bool {
        match self.last() {
            Some(c) => predicate.test(c),
            None => false,
        }
    }

    /// Scans until a character that matches the predicate is found
    /// 
    /// The window is expanded such that it contains all the matching characters
    /// but NO further characters.
    pub fn scan_while<P: CharPredicate>(&mut self, predicate: P) -> Result<(), Error> {
        let mut marker = self.end();
        while self.take_if(|c| predicate.test(c))? {
            marker = self.end();
        }
        self.backtrack(marker);
        Ok(())
    }

    /// The same as `scan_until` but inverts the predicate, such that the scan concludes when it returns `false`
    pub fn scan_until<P: CharPredicate>(&mut self, predicate: P) -> Result<(), Error> {
        self.scan_while(predicate.invert())
    }
}

pub trait CharPredicate: Sized {
    fn test(&self, c: char) -> bool;

    fn invert(self) -> InvertedCharScanPredicate<Self> {
        InvertedCharScanPredicate(self)
    }
}

impl<F: Fn(char) -> bool> CharPredicate for F {
    fn test(&self, c: char) -> bool {
        self.call((c,))
    }
}

impl CharPredicate for Range<char> {
    fn test(&self, c: char) -> bool {
        self.contains(c)
    }
}

impl CharPredicate for RangeFull {
    fn test(&self, _c: char) -> bool {
        true
    }
}

impl CharPredicate for RangeFrom<char> {
    fn test(&self, c: char) -> bool {
        self.contains(c)
    }
}

impl CharPredicate for RangeTo<char> {
    fn test(&self, c: char) -> bool {
        self.contains(c)
    }
}

impl CharPredicate for RangeToInclusive<char> {
    fn test(&self, c: char) -> bool {
        self.contains(c)
    }
}

impl CharPredicate for RangeInclusive<char> {
    fn test(&self, c: char) -> bool {
        self.contains(c)
    }
}

impl CharPredicate for char {
    fn test(&self, c: char) -> bool {
        c == *self
    }
}

pub struct InvertedCharScanPredicate<P: CharPredicate>(P);

impl<P: CharPredicate> CharPredicate for InvertedCharScanPredicate<P> {
    fn test(&self, c: char) -> bool {
        !self.0.test(c)
    }
}

#[cfg(test)]
mod tests {
    use text::TextWindow;

    macro_rules! assert_window {
        ($win: expr, $content: expr, $last: expr, $end: expr) => {
            assert_eq!($content, $win.as_str());
            assert_eq!(Some($last), $win.last());
            assert_eq!($end, $win.end());
        };
        ($win: expr, $content: expr, $end: expr) => {
            assert_eq!($content, $win.as_str());
            assert_eq!(None, $win.last());
            assert_eq!($end, $win.end());
        };
    }

    macro_rules! take_and_assert_window {
        ($win: expr, $content: expr, $last: expr, $end: expr) => {
            assert!($win.take().unwrap());
            assert_window!($win, $content, $last, $end)
        };
        ($win: expr, $content: expr, $end: expr) => {
            assert!($win.take().unwrap());
            assert_window!($win, $content, $end)
        };
    }

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
    pub fn take_loads_take_character_into_buffer() {
        let mut window = TextWindow::new("testwin");
        assert!(window.take().unwrap());
        assert_eq!("t", window.as_str());
    }

    #[test]
    pub fn take_returns_false_when_at_end_of_file() {
        let mut window = TextWindow::new("testwin");
        for _ in 0..7 {
            assert!(window.take().unwrap());
        }
        assert!(!window.take().unwrap());
        assert_eq!("testwin", window.as_str());
    }

    #[test]
    pub fn backtrack_moves_end_pointer_back_to_provided_value() {
        let mut window = TextWindow::new("testwin");
        assert!(window.take().unwrap());
        assert!(window.take().unwrap());
        assert!(window.take().unwrap());
        assert!(window.take().unwrap());
        assert_eq!("test", window.as_str());
        assert_eq!(b"test", window.as_bytes());
        let marker = window.end();
        assert!(window.take().unwrap());
        assert!(window.take().unwrap());
        assert!(window.take().unwrap());
        assert_eq!("testwin", window.as_str());
        window.backtrack(marker);
        assert_eq!("test", window.as_str());
    }

    #[test]
    pub fn advance_moves_offset_up_to_end_pointer() {
        let mut window = TextWindow::new("testwin");
        assert!(window.take().unwrap());
        assert!(window.take().unwrap());
        assert!(window.take().unwrap());
        assert!(window.take().unwrap());
        assert_eq!("test", window.as_str());
        window.advance();
        assert_eq!("", window.as_str());
        assert_eq!(None, window.last());
        assert!(window.take().unwrap());
        assert!(window.take().unwrap());
        assert!(window.take().unwrap());
        assert_eq!("win", window.as_str());
    }

    #[test]
    pub fn take_moves_in_character_increments() {
        let mut window = TextWindow::new("a¶Ё₵𐆓e\u{0301}");
        take_and_assert_window!(&mut window, "a", 'a', 1);
        take_and_assert_window!(&mut window, "a¶", '¶', 3);
        take_and_assert_window!(&mut window, "a¶Ё", 'Ё', 5);
        take_and_assert_window!(&mut window, "a¶Ё₵", '₵', 8);
        take_and_assert_window!(&mut window, "a¶Ё₵𐆓", '𐆓', 12);
        take_and_assert_window!(&mut window, "a¶Ё₵𐆓e", 'e', 13);
        take_and_assert_window!(&mut window, "a¶Ё₵𐆓e\u{0301}", '\u{0301}', 15);
    }

    #[test]
    pub fn peek_returns_false_at_eof() {
        let mut window = TextWindow::new("0");
        assert!(window.take().unwrap());
        assert!(!window.peek(..));
    }

    #[test]
    pub fn peek_returns_true_if_predicate_matches() {
        let window = TextWindow::new("0");
        assert!(window.peek('0'..='9'));
    }

    #[test]
    pub fn peek_returns_false_if_predicate_does_not_match() {
        let window = TextWindow::new("0");
        assert!(!window.peek('1'..='9'));
    }

    #[test]
    pub fn scan_while_expands_window_to_all_characters_matching_predicate() {
        let mut window = TextWindow::new("0123456789/abcdef");

        // Inclusive Range
        window.scan_while('0'..='9').unwrap();
        assert_window!(window, "0123456789", '9', 10);
        window.reset();

        // Exclusive Range
        window.scan_while('0'..'9').unwrap();
        assert_window!(window, "012345678", '8', 9);
        window.reset();

        // Inclusive RangeTo
        window.scan_while(..='9').unwrap();
        assert_window!(window, "0123456789/", '/', 11);
        window.reset();

        // Exclusive RangeTo
        window.scan_while(..'9').unwrap();
        assert_window!(window, "012345678", '8', 9);
        window.reset();

        // RangeFrom ('/' is below '0')
        window.scan_while('0'..).unwrap();
        assert_window!(window, "0123456789", '9', 10);
        window.reset();

        // RangeFull
        window.scan_while(..).unwrap();
        assert_window!(window, "0123456789/abcdef", 'f', 17);
        window.reset();

        // char
        window.scan_while('0').unwrap();
        assert_window!(window, "0", '0', 1);
        window.reset();

        // Fn
        window.scan_while(|c: char| c.to_digit(10).unwrap() < 9).unwrap();
        assert_window!(window, "012345678", '8', 9);
        window.reset();
    }

    #[test]
    pub fn take_if_only_expands_window_if_next_char_matches_predicate() {
        let mut window = TextWindow::new("0123456789/abcdef");

        // Inclusive Range
        window.take_if('0'..='9').unwrap();
        assert_window!(window, "0", '0', 1);
        window.reset();
        window.take_if('a'..='z').unwrap();
        assert_window!(window, "", 0);
        window.reset();

        // Exclusive Range
        window.take_if('0'..'9').unwrap();
        assert_window!(window, "0", '0', 1);
        window.reset();
        window.take_if('1'..'9').unwrap();
        assert_window!(window, "", 0);
        window.reset();

        // Inclusive RangeTo
        window.take_if(..='9').unwrap();
        assert_window!(window, "0", '0', 1);
        window.reset();
        window.take_if(..='/').unwrap();
        assert_window!(window, "", 0);
        window.reset();

        // Exclusive RangeTo
        window.take_if(..'1').unwrap();
        assert_window!(window, "0", '0', 1);
        window.reset();
        window.take_if(..'0').unwrap();
        assert_window!(window, "", 0);
        window.reset();

        // RangeFrom ('/' is below '0')
        window.take_if('0'..).unwrap();
        assert_window!(window, "0", '0', 1);
        window.reset();
        window.take_if('1'..).unwrap();
        assert_window!(window, "", 0);
        window.reset();

        // RangeFull
        window.take_if(..).unwrap();
        assert_window!(window, "0", '0', 1);
        window.reset();

        // char
        window.take_if('0').unwrap();
        assert_window!(window, "0", '0', 1);
        window.reset();
        window.take_if('1').unwrap();
        assert_window!(window, "", 0);
        window.reset();

        // Fn
        window.take_if(|c: char| c.to_digit(10).unwrap() < 9).unwrap();
        assert_window!(window, "0", '0', 1);
        window.reset();
        window.take_if(|c: char| c.to_digit(10).unwrap() > 0).unwrap();
        assert_window!(window, "", 0);
        window.reset();
    }
}
