#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct NilLiteral;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct BooleanLiteral {
    value: bool,
}

impl BooleanLiteral {
    pub fn value(&self) -> bool {
        self.value
    }
}

impl From<bool> for BooleanLiteral {
    fn from(value: bool) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct StringLiteral {
    value: String,
}

impl StringLiteral {
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl<T: AsRef<str>> From<T> for StringLiteral {
    fn from(value: T) -> Self {
        Self {
            value: value.as_ref().to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NumberLiteral {
    value: String,
}

impl NumberLiteral {
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Default for NumberLiteral {
    fn default() -> Self {
        Self {
            value: String::from("0"),
        }
    }
}

impl<T: AsRef<str>> From<T> for NumberLiteral {
    fn from(value: T) -> Self {
        Self {
            value: value.as_ref().to_string(),
        }
    }
}
