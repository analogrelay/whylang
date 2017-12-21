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
        match self.last() {
            '-' | '0'...'9' => self.number(),
            _ => Ok(self.emit(TokenType::Unknown, TokenValue::None))
        }
    }

    fn number(&mut self) -> Result<Token, Error> {
        let mut number_end = self.mark();
        let mut number_value: i64 = 0;

        let multiplicator = if self.last() == '-' {
            -1
        } else {
            // Not a '-' means this is the first digit.
            number_value = self.last()
                .to_digit(10)
                .expect("The caller was supposed to validate that this was a digit!") as i64;

            1
        };

        // Read until the end of the digits
        while self.take()? && self.matches('0'..='9') {
            number_value *= 10;
            number_value += multiplicator * self.last()
                .to_digit(10)
                .expect("The call to 'matches' was supposed to validate that this was a digit!") as i64;
            number_end = self.mark();
        }
        self.back(number_end);

        Ok(self.emit(TokenType::Number, TokenValue::Integer(number_value)))
    }

    // Helpers
    fn last(&self) -> char {
        self.window.last().expect("The 'last' helper should only be used when it is guaranteed that there is a character available.")
    }

    fn matches(&self, r: RangeInclusive<char>) -> bool {
        r.contains(self.last())
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

    macro_rules! single_token_test {
        ($s: expr, $typ: expr, $val: expr) => {
            let tok = get_single_token($s);
            assert_eq!(0, tok.span().start());
            assert_eq!($s.len(), tok.span().end());
            assert_eq!($typ, tok.typ());
            assert_eq!(&$val, tok.value());
        };
    }

    macro_rules! token_tests {
        ($($name: ident => $test: stmt;)*) => {
            $(
                #[test]
                pub fn $name() {
                    $test
                }
            )*
        };
    }

    token_tests! {
        literal_zero => single_token_test!("0", TokenType::Number, TokenValue::Integer(0));
        literal_pos_int => single_token_test!("123", TokenType::Number, TokenValue::Integer(123));
        literal_neg_int => single_token_test!("-123", TokenType::Number, TokenValue::Integer(-123));
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