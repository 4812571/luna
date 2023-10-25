use crate::ast::TypeSingleton;
use crate::format::{SourceFormatItem, SourceFormatSettings, SourceItem};

impl SourceFormatItem for TypeSingleton {
    fn format(&self, _settings: &SourceFormatSettings) -> SourceItem {
        SourceItem::text(self.text())
    }
}
