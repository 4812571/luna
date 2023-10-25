#[derive(Debug, Clone, PartialEq)]
pub enum TypeSingleton {
    String(String),
    Boolean(bool),
}

impl TypeSingleton {
    pub fn text(&self) -> &str {
        match self {
            Self::String(text) => &text,

            Self::Boolean(bool) => match bool {
                true => "true",
                false => "false",
            },
        }
    }
}
