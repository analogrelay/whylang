use tokenizer::{self, Token, TokenType, TokenValue};
use parser::{Expr, BinOp, Error};

pub struct Parser<I: Iterator<Item=Result<Token, tokenizer::Error>>> {
    tokens: I,
    current: Option<Result<Token, tokenizer::Error>>
}

impl<I: Iterator<Item=Result<Token, tokenizer::Error>>> Parser<I> {
    pub fn new(mut tokens: I) -> Parser<I> {
        let first = tokens.next();
        Parser {
            tokens,
            current: first
        }
    }

    pub fn expr(&mut self) -> Result<Expr, Error> {
        let primary = self.primary_expr()?;
        self.expr_rhs(primary, 0)
    }

    fn expr_rhs(&mut self, mut lhs: Expr, precedence: usize) -> Result<Expr, Error> {
        while let Some(binop) = self.peek_binop() {
            if binop.precedence() < precedence {
                // We've finished, and lhs is the finished expression
                return Ok(lhs);
            }

            // Consume the binop
            self.next();

            // Parse the next primary expression
            let mut rhs = self.primary_expr().expect("TODO: Error handling for trailing binary operator");
            if let Some(next_binop) = self.peek_binop() {
                // There's another binary operator, does it bind more strongly?
                if next_binop.precedence() > binop.precedence() {
                    // It does. Parse the right-side of this operator
                    rhs = self.expr_rhs(rhs, binop.precedence() + 1)?
                }
            }

            // Merge lhs/rhs
            lhs = Expr::binary(lhs, rhs, binop);
        }
        Ok(lhs)
    }

    fn peek_binop(&mut self) -> Option<BinOp> {
        if let Ok(t) = self.cur() {
            match t.typ() {
                TokenType::Plus => Some(BinOp::Add),
                _ => None,
            }
        } else {
            None
        }
    }

    fn primary_expr(&mut self) -> Result<Expr, Error> {
        match self.cur()?.typ() {
            TokenType::Number => self.literal(),
            _ => panic!("TODO: Implement other primary_exprs"),
        }
    }

    fn literal(&mut self) -> Result<Expr, Error> {
        let result = match self.cur()?.value() {
            &TokenValue::Int(i) => Ok(Expr::constant(i)),
            _ => panic!("TODO: Implement other literals"),
        };
        self.next();
        result
    }

    fn cur(&self) -> Result<&Token, Error> {
        match self.current {
            Some(Ok(ref t)) => Ok(t),
            Some(Err(ref e)) => Err(e.clone().into()),
            None => Err(Error::UnexpectedEndOfFile),
        }
    }

    fn next(&mut self) {
        self.current = self.tokens.next();
    }
}

#[cfg(test)]
mod tests {
    use tokenizer::{Token, TokenType, TokenValue, Tokenizer};
    use parser::{Parser, Expr, BinOp};
    use text::TextSpan;

    macro_rules! expr_tests {
        ($(
            $name: ident: $text: expr => $result: expr;
         )*) => {
           $(
               #[test]
               pub fn $name() {
                   let tokens = Tokenizer::new($text);
                   let mut parser = Parser::new(tokens);
                   let expr = parser.expr().expect("Expected the parse the succeed");
                   assert_eq!($result, expr);
               }
           )*
        };
    }

    expr_tests! {
        expr_int_literal: "42" => Expr::constant(42);
        bin_add_literals: "40 + 2" => Expr::binary(Expr::constant(40), Expr::constant(2), BinOp::Add);
        bin_add_literal_sequence: "1 + 2 + 3 + 4" =>
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