pub enum Primitive {
    Nil,
    Sting,
    Number,
    Boolean,
    Thread,
    Userdata,
}

impl Primitive {
    /// Construct a new 'nil' type.
    pub fn nil() -> Self {
        Primitive::Nil
    }

    /// Construct a new 'string' type.
    pub fn string() -> Self {
        Primitive::Sting
    }

    /// Construct a new 'number' type.
    pub fn number() -> Self {
        Primitive::Number
    }

    /// Construct a new 'boolean' type.
    pub fn boolean() -> Self {
        Primitive::Boolean
    }

    /// Construct a new 'thread' type.
    pub fn thread() -> Self {
        Primitive::Thread
    }

    /// Construct a new 'userdata' type.
    pub fn userdata() -> Self {
        Primitive::Userdata
    }

    /// The textual representation of the type.
    pub fn text(&self) -> String {
        match self {
            Primitive::Nil => "nil".to_string(),
            Primitive::Sting => "string".to_string(),
            Primitive::Number => "number".to_string(),
            Primitive::Boolean => "boolean".to_string(),
            Primitive::Thread => "thread".to_string(),
            Primitive::Userdata => "userdata".to_string(),
        }
    }
}