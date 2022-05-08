use crate::error::ToDefaultMessage;

#[derive(Debug, serde::Serialize)]
pub struct UniqueItemsErrorParams {
    items: Vec<String>,
}

impl UniqueItemsErrorParams {
    pub fn new<T>(items: &[T]) -> Self
    where
        T: std::fmt::Debug,
    {
        Self {
            items: items.iter().map(|i| format!("{:?}", i)).collect(),
        }
    }

    #[allow(dead_code)]
    pub fn items(&self) -> &Vec<String> {
        &self.items
    }
}

impl ToDefaultMessage for UniqueItemsErrorParams {
    fn to_default_message(&self) -> String {
        format!(
            "item of [{}] must be unique, but not.",
            self.items.join(", ")
        )
    }
}
