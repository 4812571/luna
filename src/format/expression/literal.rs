use crate::ast::expression::literal::{BooleanLiteral, NilLiteral, NumberLiteral, StringLiteral};
use crate::format::{SourceFormatItem, SourceFormatSettings, SourceItem};

impl SourceFormatItem for NilLiteral {
    fn format(&self, _settings: &SourceFormatSettings) -> SourceItem {
        SourceItem::text("nil")
    }
}

impl SourceFormatItem for BooleanLiteral {
    fn format(&self, _settings: &SourceFormatSettings) -> SourceItem {
        SourceItem::text(self.value().to_string())
    }
}

impl SourceFormatItem for NumberLiteral {
    fn format(&self, _settings: &SourceFormatSettings) -> SourceItem {
        SourceItem::text(self.value())
    }
}

impl SourceFormatItem for StringLiteral {
    fn format(&self, _settings: &SourceFormatSettings) -> SourceItem {
        SourceItem::text(self.value())
    }
}
