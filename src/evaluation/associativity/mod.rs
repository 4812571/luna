mod expression;
mod literal;
mod operation;
mod operator;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AssociativityValue {
    left: bool,
    right: bool,
}

impl AssociativityValue {
    pub fn none() -> Self {
        Self {
            left: false,
            right: false,
        }
    }

    pub fn left() -> Self {
        Self {
            left: true,
            right: false,
        }
    }

    pub fn right() -> Self {
        Self {
            left: false,
            right: true,
        }
    }

    pub fn full() -> Self {
        Self {
            left: true,
            right: true,
        }
    }
}

impl AssociativityValue {
    pub fn is_left(&self) -> bool {
        self.left
    }

    pub fn is_right(&self) -> bool {
        self.right
    }
}

pub trait Associativity {
    fn associativity(&self) -> AssociativityValue;
}
