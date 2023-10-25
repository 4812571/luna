#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    Negate,
    Length,
    Not,
}

impl UnaryOperator {
    /// Constructs a new `UnaryOperator::Negate`.
    pub fn negate() -> Self {
        Self::Negate
    }

    /// Constructs a new `UnaryOperator::Len`.
    pub fn len() -> Self {
        Self::Length
    }

    /// Constructs a new `UnaryOperator::Not`.
    pub fn not() -> Self {
        Self::Not
    }
}

impl UnaryOperator {
    /// Returns the textual format for the operator.
    /// ```
    /// use luna::ast::UnaryOperator;
    ///
    /// assert_eq!(UnaryOperator::negate().text(), "-");
    /// assert_eq!(UnaryOperator::len().text(), "#");
    /// assert_eq!(UnaryOperator::not().text(), "not");
    /// ````
    pub fn text(&self) -> &'static str {
        match &self {
            Self::Negate => "-",
            Self::Length => "#",
            Self::Not => "not",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    // Math
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,

    // Logical
    And,
    Or,

    // Equality
    Equal,
    NotEqual,

    // Comparisons
    LessThan,
    GreaterThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

impl BinaryOperator {
    /// Constructs a new `BinaryOperator::Add`.
    pub fn add() -> Self {
        Self::Add
    }

    /// Constructs a new `BinaryOperator::Subtract`.
    pub fn subtract() -> Self {
        Self::Subtract
    }

    /// Constructs a new `BinaryOperator::Multiply`.
    pub fn multiply() -> Self {
        Self::Multiply
    }

    /// Constructs a new `BinaryOperator::Divide`.
    pub fn divide() -> Self {
        Self::Divide
    }

    /// Constructs a new `BinaryOperator::Modulo`.
    pub fn modulo() -> Self {
        Self::Modulo
    }

    /// Constructs a new `BinaryOperator::Power`.
    pub fn power() -> Self {
        Self::Power
    }

    /// Constructs a new `BinaryOperator::And`.
    pub fn and() -> Self {
        Self::And
    }

    /// Constructs a new `BinaryOperator::Or`.
    pub fn or() -> Self {
        Self::Or
    }

    /// Constructs a new `BinaryOperator::Equal`.
    pub fn equal() -> Self {
        Self::Equal
    }

    /// Constructs a new `BinaryOperator::NotEqual`.
    pub fn not_equal() -> Self {
        Self::NotEqual
    }

    /// Constructs a new `BinaryOperator::LessThan`.
    pub fn less_than() -> Self {
        Self::LessThan
    }

    /// Constructs a new `BinaryOperator::GreaterThan`.
    pub fn greater_than() -> Self {
        Self::GreaterThan
    }

    /// Constructs a new `BinaryOperator::GreaterThanOrEqual`.
    pub fn greater_than_or_equal() -> Self {
        Self::GreaterThanOrEqual
    }

    /// Constructs a new `BinaryOperator::LessThanOrEqual`.
    pub fn less_than_or_equal() -> Self {
        Self::LessThanOrEqual
    }
}

impl BinaryOperator {
    /// Returns the textual format for the operator.
    /// ```
    /// use luna::ast::BinaryOperator;
    ///
    /// assert_eq!(BinaryOperator::add().text(), "+");
    /// assert_eq!(BinaryOperator::and().text(), "and");
    /// assert_eq!(BinaryOperator::equal().text(), "==");
    /// assert_eq!(BinaryOperator::less_than().text(), "<");
    /// ```
    pub fn text(&self) -> &'static str {
        match &self {
            // Math
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "*",
            Self::Divide => "/",
            Self::Modulo => "%",
            Self::Power => "^",

            // Logical
            Self::And => "and",
            Self::Or => "or",

            // Equality
            Self::Equal => "==",
            Self::NotEqual => "~=",

            // Comparisons
            Self::LessThan => "<",
            Self::GreaterThan => ">",
            Self::GreaterThanOrEqual => ">=",
            Self::LessThanOrEqual => "<=",
        }
    }
}

pub enum CompoundAssignmentOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
}

impl CompoundAssignmentOperator {
    pub fn text(&self) -> &'static str {
        match &self {
            Self::Add => "+=",
            Self::Subtract => "-=",
            Self::Multiply => "*=",
            Self::Divide => "/=",
            Self::Modulo => "%=",
            Self::Power => "^=",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeOperator {
    Union,
    Intersection,
}

impl TypeOperator {
    /// Constructs a new `TypeOperator::Union`.
    pub fn union() -> Self {
        Self::Union
    }

    /// Constructs a new `TypeOperator::Intersection`.
    pub fn intersection() -> Self {
        Self::Intersection
    }
}

impl TypeOperator {
    /// Returns the textual format for the operator.
    pub fn text(&self) -> &'static str {
        match &self {
            Self::Union => "|",
            Self::Intersection => "&",
        }
    }
}
