use crate::validation::error::ToDefaultMessage;

#[derive(Debug)]
pub struct MaxItemsParams {
    items: Vec<String>,
    max_items: usize,
}

impl MaxItemsParams {
    pub fn new<T>(items: &[T], max_items: usize) -> Self
    where
        T: std::fmt::Debug,
    {
        Self {
            items: items.iter().map(|i| format!("{:?}", i)).collect(),
            max_items,
        }
    }

    #[allow(dead_code)]
    pub fn items(&self) -> &Vec<String> {
        &self.items
    }

    #[allow(dead_code)]
    pub fn max_items(&self) -> usize {
        self.max_items
    }
}

impl ToDefaultMessage for MaxItemsParams {
    fn to_default_message(&self) -> String {
        format!(
            "the length of the items must be `<= {}`, but `{}`.",
            self.max_items,
            self.items.len()
        )
    }
}
