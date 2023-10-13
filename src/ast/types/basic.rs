pub enum Basic {
    Nil,
    Sting,
    Number,
    Boolean,
    Thread,
    Userdata,
}

impl Basic {
    /// Construct a new 'nil' type.
    pub fn nil() -> Self {
        Basic::Nil
    }

    /// Construct a new 'string' type.
    pub fn string() -> Self {
        Basic::Sting
    }

    /// Construct a new 'number' type.
    pub fn number() -> Self {
        Basic::Number
    }

    /// Construct a new 'boolean' type.
    pub fn boolean() -> Self {
        Basic::Boolean
    }

    /// Construct a new 'thread' type.
    pub fn thread() -> Self {
        Basic::Thread
    }

    /// Construct a new 'userdata' type.
    pub fn userdata() -> Self {
        Basic::Userdata
    }
}

impl ToString for Basic {
    fn to_string(&self) -> String {
        match self {
            Basic::Nil => "nil".to_string(),
            Basic::Sting => "string".to_string(),
            Basic::Number => "number".to_string(),
            Basic::Boolean => "boolean".to_string(),
            Basic::Thread => "thread".to_string(),
            Basic::Userdata => "userdata".to_string(),
        }
    }
}