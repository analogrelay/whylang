use tokenizer::{Token, TokenType, TokenValue};
use parser::{Expr, BinOp};

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

    pub fn expression(&mut self) -> Option<Expr> {
        if let Some(primary) = self.primary_expr() {
        } else {
            None
        }
    }

    pub fn primary(&mut self) -> Option<Expr> {
        if !self.next() {
            None
        } else {
            match self.cur().typ() {
                TokenType::Number => self.literal(),
                _ => None
            }
        }
    }

    fn literal(&mut self) -> Option<Expr> {
        match self.cur().value() {
            &TokenValue::Int(i) => Some(Expr::constant(i)),
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
    use parser::{Parser, Expr, BinOp};
    use text::TextSpan;

    macro_rules! expr_tests {
        ($(
            $name: ident: [ $(
                ($typ: expr, $val: expr)
            ),* ] => $result: expr;
         )*) => {
           $(
               #[test]
               pub fn $name() {
                   let tokens = [
                       $(
                           Token::new(TextSpan::new(0, 0), $typ, $val)
                       ),*
                   ];
                   let mut parser = Parser::new(tokens.iter().cloned());
                   let expr = parser.expression();
                   assert!(expr.is_some());
                   assert_eq!($result, expr.unwrap());
               }
           )*
        };
    }

    expr_tests! {
        expr_int_literal: [ (TokenType::Number, TokenValue::Int(42)) ] => Expr::constant(42);
        bin_add_literals: [ 
            (TokenType::Number, TokenValue::Int(40)),
            (TokenType::Plus, TokenValue::None),
            (TokenType::Number, TokenValue::Int(2))
        ] => Expr::binary(Expr::constant(40), Expr::constant(2), BinOp::Add);
    }

    // #[test]
    // pub fn expr_integer_literal() {
    //     let tokens = [
    //         Token::new(TextSpan::new(0, 0), TokenType::Number, TokenValue::Int(42))
    //     ];
    //     let mut parser = Parser::new(tokens.iter().cloned());
    //     assert_eq!(Some(Expression::Constant(Literal::Int(42))), parser.expression());
    // }
}