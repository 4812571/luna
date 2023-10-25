#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TypeBuiltIn {
    Never,
    Any,
    Unknown,
}

impl TypeBuiltIn {
    pub fn text(&self) -> &'static str {
        match self {
            Self::Never => "never",
            Self::Any => "any",
            Self::Unknown => "unknown",
        }
    }
}
