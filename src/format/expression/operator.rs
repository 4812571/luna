use crate::ast::{BinaryOperator, TypeOperator, UnaryOperator};
use crate::format::{SourceFormatItem, SourceFormatSettings, SourceItem};

impl SourceFormatItem for UnaryOperator {
    fn format(&self, settings: &SourceFormatSettings) -> SourceItem {
        let spaced = match self {
            UnaryOperator::Not => true,
            UnaryOperator::Negate => settings.operator_spacing.negate,
            UnaryOperator::Length => settings.operator_spacing.length,
        };

        match spaced {
            false => SourceItem::text(self.text()),
            true => SourceItem::text(format!(" {} ", self.text())),
        }
    }
}

impl SourceFormatItem for BinaryOperator {
    fn format(&self, settings: &SourceFormatSettings) -> SourceItem {
        let spaced = match self {
            // Math
            BinaryOperator::Add => settings.operator_spacing.add,
            BinaryOperator::Subtract => settings.operator_spacing.subtract,
            BinaryOperator::Multiply => settings.operator_spacing.multiply,
            BinaryOperator::Divide => settings.operator_spacing.divide,
            BinaryOperator::Modulo => settings.operator_spacing.modulo,
            BinaryOperator::Power => settings.operator_spacing.power,

            // Logical
            BinaryOperator::And => settings.operator_spacing.and,
            BinaryOperator::Or => settings.operator_spacing.or,

            // Equality
            BinaryOperator::Equal => settings.operator_spacing.equal,
            BinaryOperator::NotEqual => settings.operator_spacing.not_equal,

            // Comparisons
            BinaryOperator::LessThan => settings.operator_spacing.less_than,
            BinaryOperator::GreaterThan => settings.operator_spacing.greater_than,
            BinaryOperator::GreaterThanOrEqual => settings.operator_spacing.greater_than_or_equal,
            BinaryOperator::LessThanOrEqual => settings.operator_spacing.less_than_or_equal,
        };

        match spaced {
            false => SourceItem::text(self.text()),
            true => SourceItem::text(format!(" {} ", self.text())),
        }
    }
}

impl SourceFormatItem for TypeOperator {
    fn format(&self, settings: &SourceFormatSettings) -> SourceItem {
        let spaced = match self {
            TypeOperator::Intersection => settings.operator_spacing.intersection,
            TypeOperator::Union => settings.operator_spacing.union,
        };

        match spaced {
            false => SourceItem::text(self.text()),
            true => SourceItem::text(format!(" {} ", self.text())),
        }
    }
}
