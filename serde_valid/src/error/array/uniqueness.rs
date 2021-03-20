#[derive(Debug, serde::Serialize)]
pub struct UniqueItemsErrorMessage {
    items: Vec<String>,
}

impl UniqueItemsErrorMessage {
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

impl std::fmt::Display for UniqueItemsErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "item of [{}] must be unique, but not.",
            self.items.join(", ")
        )
    }
}
