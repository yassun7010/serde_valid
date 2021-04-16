#[derive(Debug, serde::Serialize, PartialEq, Eq, Hash)]
pub struct FieldName(&'static str);

impl FieldName {
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }
}

impl PartialEq<&'_ str> for FieldName {
    fn eq(&self, other: &&'_ str) -> bool {
        &self.0 == other
    }
}

impl PartialEq<str> for FieldName {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}
