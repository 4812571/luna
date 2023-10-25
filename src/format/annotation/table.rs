use itertools::Itertools;

use crate::ast::TypeArray;
use crate::ast::TypeTable;
use crate::ast::TypeTableEntry;
use crate::ast::TypeTableIndexer;
use crate::format::{SourceFormat, SourceFormatItem, SourceFormatSettings, SourceItem};

impl SourceFormatItem for TypeTableIndexer {
    fn format(&self, settings: &SourceFormatSettings) -> SourceItem {
        let index = self.index().format_string(settings);
        let value = self.value().format_string(settings);

        SourceItem::Text(format!("[{}]: {}", index, value))
    }
}

impl SourceFormatItem for TypeTableEntry {
    fn format(&self, settings: &SourceFormatSettings) -> SourceItem {
        let index = self.index();
        let value = self.value().format_string(settings);

        SourceItem::Text(format!("{}: {}", index, value))
    }
}

impl SourceFormatItem for TypeTable {
    fn format(&self, settings: &SourceFormatSettings) -> SourceItem {
        let mut entries = self
            .entries()
            .iter()
            .map(|entry| entry.format_string(settings));

        let indexer = self.indexer().as_ref().map(|a| a.format_string(settings));

        let content = match indexer {
            Some(indexer) => entries.chain(std::iter::once(indexer)).join(", "),
            None => entries.join(", "),
        };

        SourceItem::Text(format!("{{ {} }}", content))
    }
}

impl SourceFormatItem for TypeArray {
    fn format(&self, settings: &SourceFormatSettings) -> SourceItem {
        let item = self
            .item()
            .as_ref()
            .map(|a| a.format_string(settings))
            .unwrap_or("".to_string());

        SourceItem::Text(format!("{{ {} }}", item))
    }
}
