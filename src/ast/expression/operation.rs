use crate::ast::{BinaryOperator, Expression, UnaryOperator};

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOperation {
    operator: UnaryOperator,
    operand: Box<Expression>,
}

impl UnaryOperation {
    /// Constructs a new [`UnaryOperation`] with the given [`UnaryOperator`] and [`Expression`].
    pub fn new<T: Into<Expression>>(operator: UnaryOperator, operand: T) -> Self {
        Self {
            operator,
            operand: Box::new(operand.into()),
        }
    }

    /// Constructs a new negation [`UnaryOperation`] with the given [`Expression`].
    pub fn negate<T: Into<Expression>>(operand: T) -> Self {
        Self::new(UnaryOperator::Negate, operand)
    }

    /// Constructs a new length [`UnaryOperation`] with the given [`Expression`].
    pub fn length<T: Into<Expression>>(operand: T) -> Self {
        Self::new(UnaryOperator::Length, operand)
    }

    /// Constructs a new not [`UnaryOperation`] with the given [`Expression`].
    pub fn not<T: Into<Expression>>(operand: T) -> Self {
        Self::new(UnaryOperator::Not, operand)
    }
}

impl UnaryOperation {
    /// Returns the [`UnaryOperator`] of this [`UnaryOperation`].
    pub fn operator(&self) -> UnaryOperator {
        self.operator
    }

    /// Returns the [`Expression`] of this [`UnaryOperation`].
    pub fn operand(&self) -> &Expression {
        &self.operand
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOperation {
    operator: BinaryOperator,
    left: Box<Expression>,
    right: Box<Expression>,
}

impl BinaryOperation {
    /// Constructs a new [`BinaryOperation`] with the given [`BinaryOperator`] and a left and right [`Expression`].
    pub fn new<L: Into<Expression>, R: Into<Expression>>(
        operator: BinaryOperator,
        left: L,
        right: R,
    ) -> Self {
        Self {
            operator,
            left: Box::new(left.into()),
            right: Box::new(right.into()),
        }
    }

    /// Constructs a new addition [`BinaryOperation`] with the given left and right [`Expression`].
    pub fn add<L: Into<Expression>, R: Into<Expression>>(left: L, right: R) -> Self {
        Self::new(BinaryOperator::Add, left, right)
    }

    /// Constructs a new subtraction [`BinaryOperation`] with the given left and right [`Expression`].
    pub fn subtract<L: Into<Expression>, R: Into<Expression>>(left: L, right: R) -> Self {
        Self::new(BinaryOperator::Subtract, left, right)
    }

    /// Constructs a new multiplication [`BinaryOperation`] with the given left and right [`Expression`].
    pub fn multiply<L: Into<Expression>, R: Into<Expression>>(left: L, right: R) -> Self {
        Self::new(BinaryOperator::Multiply, left, right)
    }

    /// Constructs a new division [`BinaryOperation`] with the given left and right [`Expression`].
    pub fn divide<L: Into<Expression>, R: Into<Expression>>(left: L, right: R) -> Self {
        Self::new(BinaryOperator::Divide, left, right)
    }

    /// Constructs a new modulo [`BinaryOperation`] with the given left and right [`Expression`].
    pub fn modulo<L: Into<Expression>, R: Into<Expression>>(left: L, right: R) -> Self {
        Self::new(BinaryOperator::Modulo, left, right)
    }

    /// Constructs a new power [`BinaryOperation`] with the given left and right [`Expression`].
    pub fn power<L: Into<Expression>, R: Into<Expression>>(left: L, right: R) -> Self {
        Self::new(BinaryOperator::Power, left, right)
    }

    /// Constructs a new and [`BinaryOperation`] with the given left and right [`Expression`].
    pub fn and<L: Into<Expression>, R: Into<Expression>>(left: L, right: R) -> Self {
        Self::new(BinaryOperator::And, left, right)
    }

    /// Constructs a new or [`BinaryOperation`] with the given left and right [`Expression`].
    pub fn or<L: Into<Expression>, R: Into<Expression>>(left: L, right: R) -> Self {
        Self::new(BinaryOperator::Or, left, right)
    }

    /// Constructs a new equal [`BinaryOperation`] with the given left and right [`Expression`].
    pub fn equal<L: Into<Expression>, R: Into<Expression>>(left: L, right: R) -> Self {
        Self::new(BinaryOperator::Equal, left, right)
    }

    /// Constructs a new not equal [`BinaryOperation`] with the given left and right [`Expression`].
    pub fn not_equal<L: Into<Expression>, R: Into<Expression>>(left: L, right: R) -> Self {
        Self::new(BinaryOperator::NotEqual, left, right)
    }

    /// Constructs a new less than [`BinaryOperation`] with the given left and right [`Expression`].
    pub fn less_than<L: Into<Expression>, R: Into<Expression>>(left: L, right: R) -> Self {
        Self::new(BinaryOperator::LessThan, left, right)
    }

    /// Constructs a new greater than [`BinaryOperation`] with the given left and right [`Expression`].
    pub fn greater_than<L: Into<Expression>, R: Into<Expression>>(left: L, right: R) -> Self {
        Self::new(BinaryOperator::GreaterThan, left, right)
    }

    /// Constructs a new greater than or equal [`BinaryOperation`] with the given left and right [`Expression`].
    pub fn greater_than_or_equal<L: Into<Expression>, R: Into<Expression>>(
        left: L,
        right: R,
    ) -> Self {
        Self::new(BinaryOperator::GreaterThanOrEqual, left, right)
    }

    /// Constructs a new less than or equal [`BinaryOperation`] with the given left and right [`Expression`].
    pub fn less_than_or_equal<L: Into<Expression>, R: Into<Expression>>(left: L, right: R) -> Self {
        Self::new(BinaryOperator::LessThanOrEqual, left, right)
    }
}

impl BinaryOperation {
    /// Returns the [`BinaryOperator`] of this [`BinaryOperation`].
    pub fn operator(&self) -> BinaryOperator {
        self.operator
    }

    /// Returns the left [`Expression`] of this [`BinaryOperation`].
    pub fn left(&self) -> &Expression {
        &self.left
    }

    /// Returns the right [`Expression`] of this [`BinaryOperation`].
    pub fn right(&self) -> &Expression {
        &self.right
    }
}
