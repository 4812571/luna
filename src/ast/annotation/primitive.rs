#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypePrimitive {
    Nil,
    Boolean,
    Number,
    String,
    Thread,
    // Userdata,
}

impl TypePrimitive {
    /// Constructs a new `TypePrimitive::Number`.
    pub fn new() -> Self {
        Self::Number
    }

    /// Constructs a new `TypePrimitive::Nil`.
    pub fn nil() -> Self {
        Self::Nil
    }

    /// Constructs a new `TypePrimitive::Boolean`.
    pub fn boolean() -> Self {
        Self::Boolean
    }

    /// Constructs a new `TypePrimitive::String`.
    pub fn string() -> Self {
        Self::String
    }

    /// Constructs a new `TypePrimitive::Thread`.
    pub fn thread() -> Self {
        Self::Thread
    }

    // /// Constructs a new `TypePrimitive::Userdata`.
    // pub fn userdata() -> Self {
    //     Self::Userdata
    // }
}

impl TypePrimitive {
    pub fn text(&self) -> &'static str {
        match self {
            Self::Nil => "nil",
            Self::Boolean => "boolean",
            Self::Number => "number",
            Self::String => "string",
            Self::Thread => "thread",
            // Self::Userdata => "userdata",
        }
    }
}
