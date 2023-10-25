use crate::ast::TypeCombination;
use crate::format::{SourceFormat, SourceFormatItem, SourceFormatSettings, SourceItem};

impl SourceFormatItem for TypeCombination {
    fn format(&self, settings: &SourceFormatSettings) -> SourceItem {
        let operator = self.operator().text();
        let left = self.left().format_string(settings);
        let right = self.right().format_string(settings);

        SourceItem::Text(format!("{} {} {}", left, operator, right))
    }
}
