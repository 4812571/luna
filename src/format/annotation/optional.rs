use crate::ast::TypeOptional;
use crate::format::{SourceFormat, SourceFormatItem, SourceFormatSettings, SourceItem};

impl SourceFormatItem for TypeOptional {
    fn format(&self, settings: &SourceFormatSettings) -> SourceItem {
        let value = self.annotation().format_string(settings);

        SourceItem::Text(format!("{}?", value))
    }
}
