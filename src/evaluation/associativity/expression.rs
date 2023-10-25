use crate::ast::Expression;
use crate::evaluation::{Associativity, AssociativityValue};

impl Associativity for Expression {
    fn associativity(&self) -> AssociativityValue {
        match self {
            // Literal
            Self::NilLiteral(expr) => expr.associativity(),
            Self::BooleanLiteral(expr) => expr.associativity(),
            Self::NumberLiteral(expr) => expr.associativity(),
            Self::StringLiteral(expr) => expr.associativity(),

            // Operation
            Self::UnaryOperation(expr) => expr.associativity(),
            Self::BinaryOperation(expr) => expr.associativity(),
        }
    }
}
