mod built_in;
mod combination;
mod function;
mod optional;
mod primitive;
mod singleton;
mod table;

use crate::ast::TypeAnnotation;
use crate::format::{SourceFormatItem, SourceFormatSettings, SourceItem};

impl SourceFormatItem for TypeAnnotation {
    fn format(&self, settings: &SourceFormatSettings) -> SourceItem {
        match self {
            Self::BuiltIn(a) => a.format(settings),
            Self::Primitive(a) => a.format(settings),
            Self::Function(a) => a.format(settings),
            Self::Table(a) => a.format(settings),
            Self::Array(a) => a.format(settings),
            Self::Singleton(a) => a.format(settings),
            Self::Optional(a) => a.format(settings),
            Self::Combination(a) => a.format(settings),
        }
    }
}

// a and b and c and d
// ((a and b) and c) and d

// a / (b / (c / d))

// <number, number<(number): number
