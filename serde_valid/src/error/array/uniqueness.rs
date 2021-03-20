#[derive(Debug)]
pub struct UniqueItemsErrorMessage {
    items: String,
}

impl UniqueItemsErrorMessage {
    pub fn new<T>(items: &[T]) -> Self
    where
        T: std::fmt::Debug,
    {
        Self {
            items: format!("{:?}", items),
        }
    }
}

impl std::fmt::Display for UniqueItemsErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "item of {} must be unique, but not.", self.items)
    }
}
