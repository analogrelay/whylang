use std::ops::RangeInclusive;

use tokenizer::{Token, TokenType, TokenValue, Error};
use text::TextWindow;

pub struct Tokenizer<'a> {
    imp: TokenizerImpl<'a>
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Result<Token, Error>> {
        // Read the first character
        match self.imp.take() {
            Ok(true) => Some(self.imp.token()),
            Ok(false) => None,
            Err(e) => Some(Err(e))
        }
    }
}

impl<'a> Tokenizer<'a> {
    pub fn new(document: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            imp: TokenizerImpl::new(document)
        }
    }
}

struct TokenizerImpl<'a> {
    window: TextWindow<'a>
}

impl<'a> TokenizerImpl<'a> {
    pub fn new(document: &'a str) -> TokenizerImpl<'a> {
        TokenizerImpl {
            window: TextWindow::new(document)
        }
    }

    fn token(&mut self) -> Result<Token, Error> {
        match self.last().expect("We know there should be a last because callers verify it") {
            '0'...'9' => self.number(),
            _ => Ok(self.emit(TokenType::Unknown, TokenValue::None))
        }
    }

    fn number(&mut self) -> Result<Token, Error> {
        // Read until the end of the digits
        let mut number_end = self.mark();
        while self.take()? && self.matches('0'..='9') {
            number_end = self.mark();
        }
        self.back(number_end);

        Ok(self.emit(TokenType::Number, TokenValue::None))
    }

    // Helpers
    fn last(&self) -> Option<char> {
        self.window.last()
    }

    fn matches(&self, r: RangeInclusive<char>) -> bool {
        match self.last() {
            Some(c) => r.contains(c),
            None => false
        }
    }

    fn back(&mut self, marker: usize) {
        self.window.backtrack(marker);
    }

    fn mark(&self) -> usize {
        self.window.end()
    }

    fn take(&mut self) -> Result<bool, Error> {
        Ok(self.window.next()?)
    }

    fn emit(&mut self, typ: TokenType, value: TokenValue) -> Token {
        let span = self.window.span();
        self.window.advance();
        Token::new(span, typ, value)
    }
}

#[cfg(test)]
mod tests {
    use tokenizer::{Token, Tokenizer, TokenType, TokenValue};

    // macro_rules! assert_token {
    //     ($start_end: expr, $typ: expr, $val: expr, $tok: expr) => {
    //        assert_eq!($start_end.0, $tok.span().start()); 
    //        assert_eq!($start_end.1, $tok.span().start()); 
    //     };
    // }

    #[test]
    pub fn number_integer() {
        let tok = get_single_token("123");
        assert_eq!(0, tok.span().start());
        assert_eq!(3, tok.span().end());
        assert_eq!(TokenType::Number, tok.typ());
        assert_eq!(&TokenValue::Integer(123), tok.value());
    }

    fn get_single_token(s: &str) -> Token {
        let mut t = Tokenizer::new(s);
        let token = t.next()
            .expect("expected a single token")
            .expect("expected no error reading token");
        assert!(t.next().is_none(), "expected no more than one token");
        token
    }
}