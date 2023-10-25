use super::{Precedence, PrecedenceValue};
use crate::ast::{BinaryOperator, UnaryOperator};

impl Precedence for UnaryOperator {
    fn precedence(&self) -> PrecedenceValue {
        PrecedenceValue::Unary
    }
}

impl Precedence for BinaryOperator {
    fn precedence(&self) -> PrecedenceValue {
        match self {
            Self::Add => PrecedenceValue::Additive,
            Self::Subtract => PrecedenceValue::Additive,
            Self::Multiply => PrecedenceValue::Multiplicative,
            Self::Divide => PrecedenceValue::Multiplicative,
            Self::Modulo => PrecedenceValue::Multiplicative,
            Self::Power => PrecedenceValue::Exponentiation,
            Self::And => PrecedenceValue::And,
            Self::Or => PrecedenceValue::Or,
            Self::Equal => PrecedenceValue::Equality,
            Self::NotEqual => PrecedenceValue::Equality,
            Self::LessThan => PrecedenceValue::Relational,
            Self::GreaterThan => PrecedenceValue::Relational,
            Self::GreaterThanOrEqual => PrecedenceValue::Relational,
            Self::LessThanOrEqual => PrecedenceValue::Relational,
        }
    }
}
