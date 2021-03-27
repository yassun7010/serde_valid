#[derive(Debug, serde::Serialize, PartialEq, Eq, Hash)]
pub struct FieldName(String);

impl FieldName {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self(name.into())
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
