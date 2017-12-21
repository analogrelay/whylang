#[derive(Debug, PartialEq, Eq)]
pub enum Literal {
    Integer(i64)
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Lit(Literal)
}