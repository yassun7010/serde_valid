#[derive(Debug)]
pub struct UniqueItemsErrorInfo {
    items: String,
}

impl UniqueItemsErrorInfo {
    pub fn new<T>(items: &[T]) -> Self
    where
        T: std::fmt::Debug,
    {
        Self {
            items: format!("{:?}", items),
        }
    }
}

impl std::fmt::Display for UniqueItemsErrorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "item of {} must be unique, but not.", self.items)
    }
}
