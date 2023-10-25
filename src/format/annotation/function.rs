use itertools::Itertools;

use crate::ast::TypeArgument;
use crate::ast::TypeFunction;
use crate::format::{SourceFormat, SourceFormatItem, SourceFormatSettings, SourceItem};

impl SourceFormatItem for TypeArgument {
    fn format(&self, settings: &SourceFormatSettings) -> SourceItem {
        let value = self.val().format_string(settings);

        let argument = match self.name() {
            Some(name) => format!("{}: {}", name, value),
            None => format!("{}", value),
        };

        SourceItem::Text(argument)
    }
}

impl SourceFormatItem for TypeFunction {
    fn format(&self, settings: &SourceFormatSettings) -> SourceItem {
        let arguments = self
            .arguments()
            .iter()
            .map(|argument| argument.format_string(settings))
            .join(", ");

        let result = self
            .result()
            .iter()
            .map(|result| result.format_string(settings))
            .join(", ");

        // Note: this is intentionally wrapping zero arguments in parenthesis.
        // Consider the following example: (number) -> ()
        let result_has_parenthesis = self.result().len() != 1;

        match result_has_parenthesis {
            false => SourceItem::text(format!("({}) -> {}", arguments, result)),
            true => SourceItem::text(format!("({}) -> ({})", arguments, result)),
        }
    }
}
