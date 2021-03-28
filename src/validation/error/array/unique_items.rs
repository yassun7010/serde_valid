use crate::validation::error::ToDefaultMessage;

#[derive(Debug, serde::Serialize)]
pub struct UniqueItemsParams {
    items: Vec<String>,
}

impl UniqueItemsParams {
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

impl ToDefaultMessage for UniqueItemsParams {
    fn to_default_message(&self) -> String {
        format!(
            "item of [{}] must be unique, but not.",
            self.items.join(", ")
        )
    }
}
