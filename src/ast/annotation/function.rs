use crate::ast::TypeAnnotation;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeArgument {
    name: Option<String>,
    val: TypeAnnotation,
}

impl TypeArgument {
    /// Constructs a new [`TypeArgument`] with the given name and [`TypeAnnotation`].
    pub fn new<T: Into<TypeAnnotation>>(name: Option<String>, val: T) -> Self {
        Self {
            name,
            val: val.into(),
        }
    }

    /// Constructs a new anonymous [`TypeArgument`] with the given [`TypeAnnotation`].
    pub fn anonymous<T: Into<TypeAnnotation>>(val: T) -> Self {
        Self::new(None, val)
    }

    /// Constructs a new named [`TypeArgument`] with the given name and [`TypeAnnotation`].
    pub fn named<T: Into<TypeAnnotation>>(name: String, val: T) -> Self {
        Self::new(Some(name), val)
    }
}

impl TypeArgument {
    /// Returns the name of this [`TypeArgument`].
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Returns the [`TypeAnnotation`] of this [`TypeArgument`].
    pub fn val(&self) -> &TypeAnnotation {
        &self.val
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeFunction {
    arguments: Vec<TypeArgument>,
    result: Vec<TypeAnnotation>,
}

impl TypeFunction {
    pub fn arguments(&self) -> &Vec<TypeArgument> {
        &self.arguments
    }

    pub fn result(&self) -> &Vec<TypeAnnotation> {
        &self.result
    }
}
