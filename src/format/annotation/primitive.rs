use crate::ast::TypePrimitive;
use crate::format::{SourceFormatItem, SourceFormatSettings, SourceItem};

impl SourceFormatItem for TypePrimitive {
    fn format(&self, _settings: &SourceFormatSettings) -> SourceItem {
        SourceItem::text(self.text())
    }
}
