use super::{Precedence, PrecedenceValue};
use crate::ast::{BooleanLiteral, NilLiteral, NumberLiteral, StringLiteral};

impl Precedence for NilLiteral {
    fn precedence(&self) -> PrecedenceValue {
        PrecedenceValue::Verbatim
    }
}

impl Precedence for BooleanLiteral {
    fn precedence(&self) -> PrecedenceValue {
        PrecedenceValue::Verbatim
    }
}

impl Precedence for NumberLiteral {
    fn precedence(&self) -> PrecedenceValue {
        PrecedenceValue::Verbatim
    }
}

impl Precedence for StringLiteral {
    fn precedence(&self) -> PrecedenceValue {
        PrecedenceValue::Verbatim
    }
}
