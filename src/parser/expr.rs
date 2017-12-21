#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Lit {
    Int(i64)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl BinOp {
    pub fn precedence(self) -> usize {
        match self {
            BinOp::Add | BinOp::Subtract => 10,
            BinOp::Multiply | BinOp::Divide => 20,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Constant(Lit),
    Binary(Box<Expr>, Box<Expr>, BinOp),
}

impl Expr {
    pub fn constant<I: Into<Lit>>(val: I) -> Expr {
        Expr::Constant(val.into())
    }

    pub fn binary(l: Expr, r: Expr, op: BinOp) -> Expr {
        Expr::Binary(Box::new(l), Box::new(r), op)
    }

    pub fn precedence(&self) -> usize {
        match self {
            &Expr::Constant(_) => 0,
            &Expr::Binary(_, _, op) => op.precedence()
        }
    }
}

impl From<i64> for Lit {
    fn from(v: i64) -> Lit { Lit::Int(v) }
}