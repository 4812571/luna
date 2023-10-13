use crate::ast::collections::Types;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CombinationKind {
    Union,
    Intersection,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Combination {
    kind: CombinationKind,
    left: Box<Type>,
    right: Box<Type>,
}