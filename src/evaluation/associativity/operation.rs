use super::{Associativity, AssociativityValue};
use crate::ast::{BinaryOperation, UnaryOperation};

impl Associativity for UnaryOperation {
    fn associativity(&self) -> AssociativityValue {
        self.operator().associativity()
    }
}

impl Associativity for BinaryOperation {
    fn associativity(&self) -> AssociativityValue {
        self.operator().associativity()
    }
}
