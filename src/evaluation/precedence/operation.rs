use super::{Precedence, PrecedenceValue};
use crate::ast::{BinaryOperation, UnaryOperation};

impl Precedence for UnaryOperation {
    fn precedence(&self) -> PrecedenceValue {
        self.operator().precedence()
    }
}

impl Precedence for BinaryOperation {
    fn precedence(&self) -> PrecedenceValue {
        self.operator().precedence()
    }
}
