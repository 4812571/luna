use derive_more::From;

pub mod literal;
pub mod operation;

pub use literal::{BooleanLiteral, NilLiteral, NumberLiteral, StringLiteral};
pub use operation::{BinaryOperation, UnaryOperation};

#[derive(Debug, Clone, PartialEq, From)]
pub enum Expression {
    // Literals
    NilLiteral(NilLiteral),
    BooleanLiteral(BooleanLiteral),
    StringLiteral(StringLiteral),
    NumberLiteral(NumberLiteral),

    // Operations
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),
}
