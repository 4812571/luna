pub mod annotation;
pub mod expression;
pub mod settings;

pub use settings::*;

pub struct FormatDescription {
    pub indented: bool,
    pub separated: bool,
}

pub struct SourceObject {
    item: SourceItem,
    description: FormatDescription,
}

pub enum SourceItem {
    Text(String),
    Block(Vec<SourceObject>),
}

impl SourceItem {
    pub fn text<T: AsRef<str>>(value: T) -> Self {
        Self::Text(value.as_ref().to_string())
    }

    pub fn block(objects: Vec<SourceObject>) -> Self {
        Self::Block(objects)
    }

    fn indentation(settings: &SourceFormatSettings, indent: usize) -> String {
        match settings.indentation {
            IndentationMode::None => String::new(),
            IndentationMode::Spaces(spaces) => " ".repeat(spaces * indent),
            IndentationMode::Tabs(tabs) => "\t".repeat(tabs * indent),
        }
    }

    pub fn format(&self, settings: &SourceFormatSettings, indent: usize) -> String {
        let indentation = Self::indentation(settings, indent);

        let body = match self {
            Self::Text(text) => text.to_string(),

            Self::Block(objects) => {
                let mut buffer = String::new();
                let mut last_separated = false;

                for object in objects {
                    let this_separated = object.description.separated;

                    buffer.push_str(object.item.format(settings, indent + 1).as_str());

                    if last_separated || this_separated {
                        buffer.push('\n');
                    }

                    last_separated = this_separated;
                }

                buffer
            }
        };

        format!("{}{}", indentation, body)
    }
}

pub(crate) trait SourceFormatItem {
    fn format(&self, settings: &SourceFormatSettings) -> SourceItem;
}

pub trait SourceFormat {
    fn format_string(&self, settings: &SourceFormatSettings) -> String;
}

impl<T> SourceFormat for T
where
    T: SourceFormatItem,
{
    fn format_string(&self, settings: &SourceFormatSettings) -> String {
        self.format(settings).format(settings, 0)
    }
}
