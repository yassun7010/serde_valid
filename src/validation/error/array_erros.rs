use indexmap::IndexMap;

use super::{Errors, VecErrors};

#[derive(Debug, Clone, serde::Serialize, thiserror::Error)]
pub struct ArrayErrors {
    pub errors: VecErrors,
    pub items: IndexMap<usize, Errors>,
}

impl ArrayErrors {
    pub fn new(errors: VecErrors, items: IndexMap<usize, Errors>) -> Self {
        Self { errors, items }
    }

    pub fn merge(mut self, other: ArrayErrors) -> Self {
        self.errors.extend(other.errors);

        for (index, item) in other.items {
            match self.items.get_mut(&index) {
                Some(errors) => errors.merge(item),
                None => {
                    self.items.insert(index, item);
                }
            };
        }
        return self;
    }
}

impl std::fmt::Display for ArrayErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(&self) {
            Ok(json_string) => {
                write!(f, "{}", json_string)
            }
            Err(_) => Err(std::fmt::Error),
        }
    }
}
