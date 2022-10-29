use serde::ser::SerializeStruct;

use super::FlatError;

#[derive(Debug, PartialEq, Eq)]
pub struct FlatErrors(Vec<FlatError>);

impl FlatErrors {
    pub fn new(errors: impl Into<Vec<FlatError>>) -> Self {
        Self(errors.into())
    }

    pub fn errors(&self) -> &[FlatError] {
        &self.0
    }
}

impl serde::Serialize for FlatErrors {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut flat_errors = serializer.serialize_struct("FlatErrors", 1)?;
        flat_errors.serialize_field("errors", &self.0)?;
        flat_errors.end()
    }
}

impl IntoIterator for FlatErrors {
    type Item = FlatError;
    type IntoIter = <Vec<FlatError> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a FlatErrors {
    type Item = &'a FlatError;
    type IntoIter = std::slice::Iter<'a, FlatError>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use indexmap::indexmap;
    use serde_json::json;

    use crate::{
        error::{Message, ToDefaultMessage},
        flatten::IntoFlat,
        validation::{ArrayErrors, Error, Errors},
        MinItemsError,
    };

    #[test]
    fn flat_errors_json() {
        assert_eq!(
            serde_json::to_value(
                Errors::Array(ArrayErrors {
                    errors: vec![Error::MinItems(Message::new(
                        MinItemsError { min_items: 1 },
                        MinItemsError::to_default_message,
                    ))],
                    items: indexmap! {},
                })
                .into_flat()
            )
            .unwrap(),
            json!({
                "errors": [
                    {
                        "path": "",
                        "message": "The length of the items must be `>= 1`."
                    }
                ]
            })
        );
    }
}
