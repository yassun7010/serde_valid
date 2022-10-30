use jsonschema::paths::{JSONPointer, PathChunk};

use crate::{
    error::{Message, ToDefaultMessage},
    validation::{ArrayErrors, ItemErrorsMap, ObjectErrors, PropertyErrorsMap},
};

use super::{FlatError, FlatErrors};

pub trait IntoFlat
where
    Self: Sized,
{
    fn into_flat(self) -> FlatErrors {
        self.into_flat_at(&JSONPointer::default())
    }

    fn into_flat_at(self, path: &JSONPointer) -> FlatErrors;
}

impl IntoFlat for crate::validation::Errors {
    fn into_flat_at(self, path: &JSONPointer) -> FlatErrors {
        match self {
            crate::validation::Errors::Array(errors) => errors.into_flat_at(path),
            crate::validation::Errors::Object(errors) => errors.into_flat_at(path),
            crate::validation::Errors::NewType(errors) => errors.into_flat_at(path),
        }
    }
}

impl IntoFlat for crate::validation::Error {
    fn into_flat_at(self, path: &JSONPointer) -> FlatErrors {
        match self {
            crate::validation::Error::Minimum(inner) => inner.into_flat_at(path),
            crate::validation::Error::Maximum(inner) => inner.into_flat_at(path),
            crate::validation::Error::ExclusiveMinimum(inner) => inner.into_flat_at(path),
            crate::validation::Error::ExclusiveMaximum(inner) => inner.into_flat_at(path),
            crate::validation::Error::MultipleOf(inner) => inner.into_flat_at(path),
            crate::validation::Error::MinLength(inner) => inner.into_flat_at(path),
            crate::validation::Error::MaxLength(inner) => inner.into_flat_at(path),
            crate::validation::Error::Pattern(inner) => inner.into_flat_at(path),
            crate::validation::Error::MinItems(inner) => inner.into_flat_at(path),
            crate::validation::Error::MaxItems(inner) => inner.into_flat_at(path),
            crate::validation::Error::UniqueItems(inner) => inner.into_flat_at(path),
            crate::validation::Error::MinProperties(inner) => inner.into_flat_at(path),
            crate::validation::Error::MaxProperties(inner) => inner.into_flat_at(path),
            crate::validation::Error::Enumerate(inner) => inner.into_flat_at(path),
            crate::validation::Error::Items(inner) => inner.into_flat_at(path),
            crate::validation::Error::Properties(inner) => inner.into_flat_at(path),
            crate::validation::Error::Custom(inner) => {
                FlatErrors::new(vec![FlatError::new(path.to_owned(), inner)])
            }
        }
    }
}

impl IntoFlat for Vec<crate::validation::Error> {
    fn into_flat_at(self, path: &JSONPointer) -> FlatErrors {
        FlatErrors::new(self.into_iter().fold(vec![], |pre, error| {
            pre.into_iter()
                .chain(error.into_flat_at(path))
                .collect::<Vec<_>>()
        }))
    }
}

impl IntoFlat for ItemErrorsMap {
    fn into_flat_at(self, path: &JSONPointer) -> FlatErrors {
        FlatErrors::new(self.into_iter().fold(vec![], |pre, (index, errors)| {
            pre.into_iter()
                .chain(errors.into_flat().into_iter().map(|e| {
                    e.merge_childs(path.clone().into_iter().chain([PathChunk::Index(index)]))
                }))
                .collect::<Vec<_>>()
        }))
    }
}

impl IntoFlat for PropertyErrorsMap {
    fn into_flat_at(self, path: &JSONPointer) -> FlatErrors {
        FlatErrors::new(self.into_iter().fold(vec![], |pre, (property, errors)| {
            pre.into_iter()
                .chain(errors.into_flat().into_iter().map(|error| {
                    error.merge_childs(
                        path.clone()
                            .into_iter()
                            .chain([PathChunk::Property(property.to_string().into_boxed_str())]),
                    )
                }))
                .collect::<Vec<_>>()
        }))
    }
}

impl<T> IntoFlat for Message<T>
where
    T: ToDefaultMessage,
{
    fn into_flat_at(self, path: &JSONPointer) -> FlatErrors {
        FlatErrors::new(vec![FlatError::new(
            path.to_owned(),
            self.error().to_default_message(),
        )])
    }
}

impl IntoFlat for ArrayErrors {
    fn into_flat_at(self, path: &JSONPointer) -> FlatErrors {
        FlatErrors::new(
            self.errors
                .into_flat_at(path)
                .into_iter()
                .chain(self.items.into_flat_at(path))
                .collect::<Vec<_>>(),
        )
    }
}

impl IntoFlat for ObjectErrors {
    fn into_flat_at(self, path: &JSONPointer) -> FlatErrors {
        FlatErrors::new(
            self.errors
                .into_flat_at(path)
                .into_iter()
                .chain(self.properties.into_flat_at(path))
                .collect::<Vec<_>>(),
        )
    }
}

impl From<Vec<FlatError>> for FlatErrors {
    fn from(errors: Vec<FlatError>) -> Self {
        Self::new(errors)
    }
}

#[cfg(test)]
mod tests {
    use indexmap::indexmap;
    use serde_valid_literal::Number;

    use super::*;

    use crate::{
        validation::{Error, Errors},
        MaximumError, MinItemsError,
    };

    #[test]
    fn array_errors_flatten() {
        let min_items = Message::new(
            MinItemsError { min_items: 1 },
            MinItemsError::to_default_message,
        );
        let maximum = Message::new(
            MaximumError {
                maximum: Number::I32(1),
            },
            MaximumError::to_default_message,
        );
        assert_eq!(
            Errors::Array(ArrayErrors {
                errors: vec![Error::MinItems(min_items.clone())],
                items: indexmap! {
                    0 => Errors::Array(
                        ArrayErrors {
                            errors: vec![Error::Maximum(maximum.clone())],
                            items: indexmap! {
                                2 => Errors::NewType(vec![Error::Maximum(maximum.clone())]),
                            }
                        }
                    ),
                    3 => Errors::NewType(vec![Error::Maximum(maximum.clone())]),
                    5 => Errors::Object(
                        ObjectErrors {
                            errors: vec![Error::Maximum(maximum.clone())],
                            properties: indexmap! {
                                "name" => Errors::NewType(vec![Error::Maximum(maximum.clone())]),
                            }
                        }
                    ),

                },
            })
            .into_flat(),
            FlatErrors::new(vec![
                FlatError::new(
                    JSONPointer::default(),
                    min_items.error().to_default_message(),
                ),
                FlatError::new(
                    JSONPointer::from([PathChunk::from(0)].as_ref()),
                    maximum.error().to_default_message(),
                ),
                FlatError::new(
                    JSONPointer::from([PathChunk::from(0), PathChunk::from(2)].as_ref()),
                    maximum.error().to_default_message(),
                ),
                FlatError::new(
                    JSONPointer::from([PathChunk::from(3)].as_ref()),
                    maximum.error().to_default_message(),
                ),
                FlatError::new(
                    JSONPointer::from([PathChunk::from(5)].as_ref()),
                    maximum.error().to_default_message(),
                ),
                FlatError::new(
                    JSONPointer::from(
                        [PathChunk::from(5), PathChunk::from("name".to_owned())].as_ref()
                    ),
                    maximum.error().to_default_message(),
                )
            ])
        );
    }
}
