use crate::ast::TypeBuiltIn;
use crate::format::{SourceFormatItem, SourceFormatSettings, SourceItem};

impl SourceFormatItem for TypeBuiltIn {
    fn format(&self, _settings: &SourceFormatSettings) -> SourceItem {
        SourceItem::text(self.text())
    }
}
