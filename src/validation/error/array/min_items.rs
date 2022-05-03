use crate::validation::error::ToDefaultMessage;

#[derive(Debug)]
pub struct MinItemsParams {
    items: Vec<String>,
    min_items: usize,
}

impl MinItemsParams {
    pub fn new<T>(items: &[T], min_items: usize) -> Self
    where
        T: std::fmt::Debug,
    {
        Self {
            items: items.iter().map(|i| format!("{:?}", i)).collect(),
            min_items,
        }
    }

    #[allow(dead_code)]
    pub fn items(&self) -> &Vec<String> {
        &self.items
    }

    #[allow(dead_code)]
    pub fn min_items(&self) -> usize {
        self.min_items
    }
}

impl ToDefaultMessage for MinItemsParams {
    fn to_default_message(&self) -> String {
        format!(
            "the length of the items must be `>= {}`, but `{}`.",
            self.min_items,
            self.items.len()
        )
    }
}
