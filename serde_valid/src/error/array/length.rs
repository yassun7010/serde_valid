#[derive(Debug)]
pub struct ItemsErrorMessage {
    items: String,
    items_length: usize,
    min_items: Option<usize>,
    max_items: Option<usize>,
}

impl ItemsErrorMessage {
    pub fn new<T>(items: &[T], min_items: Option<usize>, max_items: Option<usize>) -> Self
    where
        T: std::fmt::Debug,
    {
        Self {
            items: format!("{:?}", items),
            items_length: items.len(),
            min_items,
            max_items,
        }
    }
}

impl std::fmt::Display for ItemsErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_items = match &self.min_items {
            Some(items) => format!("{} <= ", items),
            None => String::new(),
        };
        let max_items = match &self.max_items {
            Some(items) => format!(" <= {}", items),
            None => String::new(),
        };
        write!(
            f,
            "items length of {} must be in `{}length{}`, but `{}`.",
            self.items, min_items, max_items, self.items_length
        )
    }
}
