use text::Error;
use utils;

/// Represents a sliding window of text.
pub struct TextWindow<'a> {
    buf: &'a str,
    offset: usize,
    end: usize,
}

impl<'a> TextWindow<'a> {
    pub fn new(buf: &'a str) -> TextWindow<'a> {
        TextWindow {
            buf,
            offset: 0,
            end: 0,
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn end(&self) -> usize {
        self.end
    }

    /// Gets a `str` that represents the current content of the window
    pub fn as_str(&self) -> &str {
        &self.buf[self.offset..self.end]
    }

    /// Load another character into the buffer
    ///
    /// ## Returns
    /// `true` if a character is successfully read in.
    /// `false` if end-of-file has been reached.
    pub fn next(&mut self) -> bool {
        if self.end >= self.buf.len() {
            false
        } else {
            // See how many bytes are needed to load the character pointed to by `end`
            let width = utils::utf8_char_width(self.buf.as_bytes()[self.end]);

            // Advance that number of bytes
            let new_end = self.end + width;

            self.end = new_end;
            true
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
            "The requested index does not represent a character boundary."
        );
        self.end = new_end;
    }
}

#[cfg(test)]
mod tests {
    use text::TextWindow;

    #[test]
    pub fn as_str_returns_empty_string_when_window_initialized() {
        assert_eq!("", create_window().as_str());
    }

    #[test]
    pub fn next_loads_next_character_into_buffer() {
        let mut window = create_window();
        assert!(window.next());
        assert_eq!("t", window.as_str());
    }

    #[test]
    pub fn next_returns_false_when_at_end_of_file() {
        let mut window = create_window();
        for _ in 0..22 {
            assert!(window.next());
        }
        assert!(!window.next());
        assert_eq!("this is a test window!", window.as_str());
    }

    #[test]
    pub fn backtrack_moves_end_pointer_back_to_provided_value() {
        let mut window = create_window();
        assert!(window.next());
        assert!(window.next());
        assert!(window.next());
        assert!(window.next());
        assert_eq!("this", window.as_str());
        let marker = window.end();
        assert!(window.next());
        assert!(window.next());
        assert!(window.next());
        assert_eq!("this is", window.as_str());
        window.backtrack(marker);
        assert_eq!("this", window.as_str());
    }

    fn create_window() -> TextWindow<'static> {
        TextWindow::new("this is a test window!")
    }
}
