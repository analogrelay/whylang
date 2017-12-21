use tokenizer::{Token, TokenType, TokenValue};
use parser::{Expr, BinOp};

pub struct Parser<I: Iterator<Item=Token>> {
    tokens: I,
    current: Option<Token>
}

impl<I: Iterator<Item=Token>> Parser<I> {
    pub fn new(mut tokens: I) -> Parser<I> {
        let first = tokens.next();
        Parser {
            tokens,
            current: first
        }
    }

    pub fn expr(&mut self) -> Option<Expr> {
        // Parse the left side
        if let Some(primary) = self.primary_expr() {
            self.expr_rhs(primary, 0)
        } else {
            None
        }
    }

    fn expr_rhs(&mut self, mut lhs: Expr, precedence: usize) -> Option<Expr> {
        while let Some(binop) = self.peek_binop() {
            if binop.precedence() < precedence {
                // We've finished, and lhs is the finished expression
                return Some(lhs);
            }

            // Consume the binop
            self.next();

            // Parse the next primary expression
            let mut rhs = self.primary_expr().expect("TODO: Error handling for trailing binary operator");
            if let Some(next_binop) = self.peek_binop() {
                // There's another binary operator, does it bind more strongly?
                if next_binop.precedence() > binop.precedence() {
                    // It does. Parse the right-side of this operator
                    rhs = self.expr_rhs(rhs, binop.precedence() + 1)
                        .expect("TODO: Error handling for trailing binary operator")
                }
            }

            // Merge lhs/rhs
            lhs = Expr::binary(lhs, rhs, binop);
        }
        Some(lhs)
    }

    fn peek_binop(&mut self) -> Option<BinOp> {
        match self.cur()?.typ() {
            TokenType::Plus => Some(BinOp::Add),
            _ => None
        }
    }

    fn primary_expr(&mut self) -> Option<Expr> {
        match self.cur()?.typ() {
            TokenType::Number => self.literal(),
            _ => None
        }
    }

    fn literal(&mut self) -> Option<Expr> {
        let result = match self.cur()?.value() {
            &TokenValue::Int(i) => Some(Expr::constant(i)),
            _ => None
        };
        self.next();
        result
    }

    fn cur(&self) -> Option<&Token> {
        self.current.as_ref()
    }

    fn next(&mut self) {
        self.current = self.tokens.next();
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
                   let expr = parser.expr();
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
        bin_add_literal_sequence: [ 
            (TokenType::Number, TokenValue::Int(1)),
            (TokenType::Plus, TokenValue::None),
            (TokenType::Number, TokenValue::Int(2)),
            (TokenType::Plus, TokenValue::None),
            (TokenType::Number, TokenValue::Int(3)),
            (TokenType::Plus, TokenValue::None),
            (TokenType::Number, TokenValue::Int(4))
        ] => 
            Expr::binary(
                Expr::binary(
                    Expr::binary(
                        Expr::constant(1),
                        Expr::constant(2),
                        BinOp::Add),
                    Expr::constant(3),
                    BinOp::Add),
                Expr::constant(4),
                BinOp::Add);
    }
}