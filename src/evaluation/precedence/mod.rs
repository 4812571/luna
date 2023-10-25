mod expression;
mod literal;
mod operation;
mod operator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrecedenceValue {
    Verbatim,
    Unary,
    Exponentiation,
    Multiplicative,
    Additive,
    And,
    Or,
    Equality,
    Relational,
}

pub trait Precedence {
    fn precedence(&self) -> PrecedenceValue;
}
