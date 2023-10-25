use crate::ast::{TypeAnnotation, TypeOperator};

#[derive(Debug, Clone, PartialEq)]
pub struct TypeCombination {
    operator: TypeOperator,
    left: Box<TypeAnnotation>,
    right: Box<TypeAnnotation>,
}

impl TypeCombination {
    /// Constructs a new [`TypeCombination`] with the given [`TypeOperator`] and a left and right [`TypeAnnotation`].
    pub fn new<L: Into<TypeAnnotation>, R: Into<TypeAnnotation>>(
        operator: TypeOperator,
        left: L,
        right: R,
    ) -> Self {
        Self {
            operator,
            left: Box::new(left.into()),
            right: Box::new(right.into()),
        }
    }

    /// Returns the [`TypeOperator`] of this [`TypeCombination`].
    pub fn operator(&self) -> TypeOperator {
        self.operator
    }

    /// Returns the left [`TypeAnnotation`] of this [`TypeCombination`].
    pub fn left(&self) -> &TypeAnnotation {
        &self.left
    }

    /// Returns the right [`TypeAnnotation`] of this [`TypeCombination`].
    pub fn right(&self) -> &TypeAnnotation {
        &self.right
    }
}
