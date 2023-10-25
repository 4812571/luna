use crate::ast::Expression;
use crate::format::{SourceFormatItem, SourceFormatSettings, SourceItem};

mod literal;
mod operation;
mod operator;

impl SourceFormatItem for Expression {
    fn format(&self, settings: &SourceFormatSettings) -> SourceItem {
        match self {
            Expression::NilLiteral(expr) => expr.format(settings),
            Expression::BooleanLiteral(expr) => expr.format(settings),
            Expression::NumberLiteral(expr) => expr.format(settings),
            Expression::StringLiteral(expr) => expr.format(settings),

            Expression::UnaryOperation(expr) => expr.format(settings),
            Expression::BinaryOperation(expr) => expr.format(settings),
        }
    }
}
