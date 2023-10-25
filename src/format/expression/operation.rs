use crate::ast::{BinaryOperation, UnaryOperation};
use crate::evaluation::{Associativity, EvaluationOrder};
use crate::format::{SourceFormat, SourceFormatItem, SourceFormatSettings, SourceItem};

impl SourceFormatItem for UnaryOperation {
    fn format(&self, settings: &SourceFormatSettings) -> SourceItem {
        let operator = self.operator().format_string(settings);
        let operand = self.operand().format_string(settings);

        match self.should_wrap(self.operand(), self.associativity().is_right()) {
            true => SourceItem::text(format!("{}({})", operator, operand)),
            false => SourceItem::text(format!("{}{}", operator, operand)),
        }
    }
}

impl SourceFormatItem for BinaryOperation {
    fn format(&self, settings: &SourceFormatSettings) -> SourceItem {
        let operator = self.operator().format_string(settings);

        let left = self.left().format_string(settings);
        let right = self.right().format_string(settings);

        let left_should_wrap = self.should_wrap(self.left(), self.associativity().is_right());
        let right_should_wrap = self.should_wrap(self.right(), self.associativity().is_left());

        let left_string = match left_should_wrap {
            true => format!("({})", left),
            false => left,
        };

        let right_string = match right_should_wrap {
            true => format!("({})", right),
            false => right,
        };

        SourceItem::text(format!("{}{}{}", left_string, operator, right_string))
    }
}
