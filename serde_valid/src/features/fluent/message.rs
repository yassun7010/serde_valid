use fluent_0::FluentValue;

use crate::validation::DefaultFormat;

#[derive(Debug, Clone)]
pub struct Message {
    pub id: &'static str,
    pub args: Vec<(&'static str, FluentValue<'static>)>,
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)
    }
}

impl DefaultFormat for Message {
    #[inline]
    fn default_format(&self) -> String {
        self.id.to_string()
    }
}
