use crate::ast::Expression;
use crate::evaluation::{Precedence, PrecedenceValue};

impl Precedence for Expression {
    fn precedence(&self) -> PrecedenceValue {
        match self {
            // Literal
            Self::NilLiteral(expr) => expr.precedence(),
            Self::BooleanLiteral(expr) => expr.precedence(),
            Self::NumberLiteral(expr) => expr.precedence(),
            Self::StringLiteral(expr) => expr.precedence(),

            // Operation
            Self::UnaryOperation(expr) => expr.precedence(),
            Self::BinaryOperation(expr) => expr.precedence(),
        }
    }
}
