pub enum IndentationMode {
    None,
    Spaces(usize),
    Tabs(usize),
}

impl Default for IndentationMode {
    fn default() -> Self {
        Self::Spaces(4)
    }
}

pub struct OperatorSpacing {
    // Types
    pub union: bool,
    pub intersection: bool,

    // Unary
    pub negate: bool,
    pub length: bool,

    // Math
    pub add: bool,
    pub subtract: bool,
    pub multiply: bool,
    pub divide: bool,
    pub modulo: bool,
    pub power: bool,

    // Logical
    pub and: bool,
    pub or: bool,

    // Equality
    pub equal: bool,
    pub not_equal: bool,

    // Comparisons
    pub less_than: bool,
    pub greater_than: bool,
    pub greater_than_or_equal: bool,
    pub less_than_or_equal: bool,
}

impl Default for OperatorSpacing {
    fn default() -> Self {
        Self {
            // Types
            union: true,
            intersection: true,

            // Unary
            negate: false,
            length: false,

            // Math
            add: true,
            subtract: true,
            multiply: true,
            divide: true,
            modulo: true,
            power: true,

            // Logical
            and: true,
            or: true,

            // Equality
            equal: true,
            not_equal: true,

            // Comparisons
            less_than: true,
            greater_than: true,
            greater_than_or_equal: true,
            less_than_or_equal: true,
        }
    }
}

#[derive(Default)]
pub struct SourceFormatSettings {
    pub indentation: IndentationMode,
    pub operator_spacing: OperatorSpacing,
}
