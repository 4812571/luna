use crate::ast::TypeAnnotation;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeOptional {
    annotation: Box<TypeAnnotation>,
}

impl TypeOptional {
    /// Constructs a new [`Optional`] with the given [`TypeAnnotation`].
    pub fn new<T: Into<TypeAnnotation>>(annotation: T) -> Self {
        Self {
            annotation: Box::new(annotation.into()),
        }
    }

    /// Returns the [`TypeAnnotation`] of this [`Optional`].
    pub fn annotation(&self) -> &TypeAnnotation {
        &self.annotation
    }
}
