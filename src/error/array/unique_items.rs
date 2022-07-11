use crate::error::ToDefaultMessage;

#[derive(Debug, Clone, serde::Serialize)]
pub struct UniqueItemsErrorParams {}

impl UniqueItemsErrorParams {
    pub fn new() -> Self {
        Self {}
    }
}

impl ToDefaultMessage for UniqueItemsErrorParams {
    fn to_default_message(&self) -> String {
        format!("items must be unique.")
    }
}
