use tokenizer::{Token, TokenType, TokenValue};
use parser::{Expression, Literal};

pub struct Parser<I: Iterator<Item=Token>> {
    tokens: I,
    current: Option<Token>
}

impl<I: Iterator<Item=Token>> Parser<I> {
    pub fn new(tokens: I) -> Parser<I> {
        Parser {
            tokens,
            current: None
        }
    }

    pub fn expression(&mut self) -> Option<Expression> {
        if !self.next() {
            None
        } else {
            match self.cur().typ() {
                TokenType::Number => self.literal(),
                _ => None
            }
        }
    }

    fn literal(&mut self) -> Option<Expression> {
        match self.cur().value() {
            &TokenValue::Integer(i) => Some(Expression::Lit(Literal::Integer(i))),
            _ => None
        }
    }

    fn cur(&self) -> &Token {
        self.current.as_ref().unwrap()
    }

    fn next(&mut self) -> bool {
        self.current = self.tokens.next();
        self.current.is_some()
    }
}

#[cfg(test)]
mod tests {
    use tokenizer::{Token, TokenType, TokenValue};
    use parser::{Parser, Expression, Literal};
    use text::TextSpan;

    #[test]
    pub fn expr_integer_literal() {
        let tokens = [
            Token::new(TextSpan::new(0, 0), TokenType::Number, TokenValue::Integer(42))
        ];
        let mut parser = Parser::new(tokens.iter().cloned());
        assert_eq!(Some(Expression::Lit(Literal::Integer(42))), parser.expression());
    }
}