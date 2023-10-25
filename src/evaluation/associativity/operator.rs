use super::{Associativity, AssociativityValue};
use crate::ast::{BinaryOperator, UnaryOperator};

// Some notes on associativity:

// Short-circuiting operators always evaluate the same values, regardless of placement of parentehsis
// a() and b() and c() is the same as (a() and b()) and c() is the same as a() and (b() and c())
// In all cases:
// a() is evaluated,
// if a() is true, b() is evaluated
// if b() is true, c() is evaluated

// Comparison operators are left associative, however, this will commonly error.
// However, in the case of metatables, this is always the case.
// a > b > c // typically errors if 'a > b' is a boolean.

// All unary operators are right associative.
// #-a is the same as #(-a).

impl Associativity for UnaryOperator {
    fn associativity(&self) -> AssociativityValue {
        AssociativityValue::right()
    }
}

impl Associativity for BinaryOperator {
    fn associativity(&self) -> AssociativityValue {
        match self {
            Self::Add => AssociativityValue::full(),
            Self::Subtract => AssociativityValue::left(),
            Self::Multiply => AssociativityValue::full(),
            Self::Divide => AssociativityValue::left(),
            Self::Modulo => AssociativityValue::left(),
            Self::Power => AssociativityValue::right(),
            Self::And => AssociativityValue::full(),
            Self::Or => AssociativityValue::full(),
            Self::Equal => AssociativityValue::left(),
            Self::NotEqual => AssociativityValue::left(),
            Self::LessThan => AssociativityValue::left(),
            Self::GreaterThan => AssociativityValue::left(),
            Self::GreaterThanOrEqual => AssociativityValue::left(),
            Self::LessThanOrEqual => AssociativityValue::left(),
        }
    }
}
