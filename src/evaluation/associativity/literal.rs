use super::{Associativity, AssociativityValue};
use crate::ast::{BooleanLiteral, NilLiteral, NumberLiteral, StringLiteral};

impl Associativity for NilLiteral {
    fn associativity(&self) -> AssociativityValue {
        AssociativityValue::none()
    }
}

impl Associativity for BooleanLiteral {
    fn associativity(&self) -> AssociativityValue {
        AssociativityValue::none()
    }
}

impl Associativity for NumberLiteral {
    fn associativity(&self) -> AssociativityValue {
        AssociativityValue::none()
    }
}

impl Associativity for StringLiteral {
    fn associativity(&self) -> AssociativityValue {
        AssociativityValue::none()
    }
}
