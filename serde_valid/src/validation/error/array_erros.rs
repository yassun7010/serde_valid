use serde::ser::SerializeStruct;

use super::{ItemErrorsMap, VecErrors};

#[derive(Debug, Clone, thiserror::Error)]
pub struct ArrayErrors<Err = crate::validation::Error> {
    pub errors: VecErrors<Err>,
    pub items: ItemErrorsMap<Err>,
}

impl<Err> serde::Serialize for ArrayErrors<Err>
where
    Err: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut array_errors = serializer.serialize_struct("ArrayErrors", 2)?;
        array_errors.serialize_field("errors", &self.errors)?;
        array_errors.serialize_field("items", &self.items)?;
        array_errors.end()
    }
}

impl<Err> ArrayErrors<Err> {
    pub fn new(errors: VecErrors<Err>, items: ItemErrorsMap<Err>) -> Self {
        Self { errors, items }
    }
}

impl<Err> ArrayErrors<Err>
where
    Err: Clone,
{
    pub fn merge(mut self, other: ArrayErrors<Err>) -> Self {
        self.errors.extend(other.errors);

        for (index, item) in other.items {
            match self.items.get_mut(&index) {
                Some(errors) => errors.merge(item),
                None => {
                    self.items.insert(index, item);
                }
            };
        }
        self
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
