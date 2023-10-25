use crate::ast::{Binding, Expression};

pub struct LocalAssign {
    bindings: Vec<Binding>,
    values: Vec<Expression>,
}

impl LocalAssign {
    pub fn assign_many(bindings: &[Binding], values: &[Expression]) -> Self {
        Self {
            bindings: bindings.to_vec(),
            values: values.to_vec(),
        }
    }

    pub fn declare_many(bindings: &[Binding]) -> Self {
        self::assign_many(bindings, &[])
    }

    pub fn assign_one(binding: Binding, value: Expression) -> Self {
        self::assign_many(&[binding], &[value])
    }

    pub fn declare_one(binding: Binding) -> Self {
        self::declare_many(&[binding])
    }
}