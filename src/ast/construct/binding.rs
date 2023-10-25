use crate::ast::TypeAnnotation;

#[derive(Debug, Clone, PartialEq)]
pub struct Binding {
    pub name: String,
    pub annotation: Option<TypeAnnotation>,
}

impl Binding {
    pub fn new<T: AsRef<str>>(name: T, annotation: Option<TypeAnnotation>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            annotation,
        }
    }

    pub fn named<T: AsRef<str>>(name: T) -> Self {
        Self::new(name, None)
    }

    pub fn annotated<T: AsRef<str>>(name: T, annotation: TypeAnnotation) -> Self {
        Self::new(name, Some(annotation))
    }
}
