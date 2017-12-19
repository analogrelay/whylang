use std::ops::Range;

use tokenizer::{Token, TokenType, TokenValue};
use text::TextWindow;

pub struct Tokenizer<'a> {
    window: TextWindow<'a>
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.next_token()
    }
}

impl<'a> Tokenizer<'a> {
    pub fn new(document: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            window: TextWindow::new(document)
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        // Read the first character
        if !self.next_char() {
            None
        } else {
            Some(self.token())
        }
    }

    fn token(&mut self) -> Token {
        match self.window.as_bytes()[0] {
            b'0'...b'9' => self.number(),
            _ => self.emit(TokenType::Unknown, TokenValue::None)
        }
    }

    fn number(&mut self) -> Token {
        unimplemented!();
    }

    // Helpers
    fn next_char(&mut self) -> bool {
        self.window.next()
    }

    fn emit(&mut self, typ: TokenType, value: TokenValue) -> Token {
        let span = self.window.span();
        self.window.advance();
        Token::new(span, typ, value)
    }
}