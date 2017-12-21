use tokenizer::{Token, TokenType, TokenValue, Error};
use text::TextWindow;

pub struct Tokenizer<'a> {
    imp: TokenizerImpl<'a>
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Result<Token, Error>> {
        // Read the first character
        match self.imp.win.take() {
            Ok(true) => Some(self.imp.token()),
            Ok(false) => None,
            Err(e) => Some(Err(e.into()))
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
    win: TextWindow<'a>
}

impl<'a> TokenizerImpl<'a> {
    pub fn new(document: &'a str) -> TokenizerImpl<'a> {
        TokenizerImpl {
            win: TextWindow::new(document)
        }
    }

    fn token(&mut self) -> Result<Token, Error> {
        match self.win.last().unwrap() {
            '-' | '0'...'9' => self.number(),
            '_' | 'a'...'z' | 'A'...'Z' => self.identifier(),
            _ => Ok(self.emit(TokenType::Unknown, TokenValue::None))
        }
    }

    fn identifier(&mut self) -> Result<Token, Error> {
        self.win.scan_while(|c| match c {
            '_' | 'a'...'z' | 'A'...'Z' | '0'...'9' => true,
            _ => false
        })?;

        let ident = self.win.as_str().into();
        Ok(self.emit(TokenType::Identifier, TokenValue::Symbol(ident)))
    }

    fn number(&mut self) -> Result<Token, Error> {
        if self.win.last_is('-') {
            self.win.take()?;
        }

        // Read all the digits
        self.win.scan_while('0'..='9')?;

        // Parse the number
        let num: i64 = self.win.as_str().parse()?;

        Ok(self.emit(TokenType::Number, TokenValue::Integer(num)))
    }

    fn emit(&mut self, typ: TokenType, value: TokenValue) -> Token {
        let span = self.win.span();
        self.win.advance();
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
        identifier => single_token_test!("_123foo_bar", TokenType::Identifier, TokenValue::Symbol("_123foo_bar".into()));
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