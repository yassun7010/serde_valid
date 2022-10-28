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

    fn into_flat_at(self, pointer: &JSONPointer) -> FlatErrors;
}

impl IntoFlat for crate::validation::Errors {
    fn into_flat_at(self, pointer: &JSONPointer) -> FlatErrors {
        match self {
            crate::validation::Errors::Array(errors) => errors.into_flat_at(pointer),
            crate::validation::Errors::Object(errors) => errors.into_flat_at(pointer),
            crate::validation::Errors::NewType(errors) => errors.into_flat_at(pointer),
        }
    }
}

impl IntoFlat for crate::validation::Error {
    fn into_flat_at(self, pointer: &JSONPointer) -> FlatErrors {
        match self {
            crate::validation::Error::Minimum(inner) => inner.into_flat_at(pointer),
            crate::validation::Error::Maximum(inner) => inner.into_flat_at(pointer),
            crate::validation::Error::ExclusiveMinimum(inner) => inner.into_flat_at(pointer),
            crate::validation::Error::ExclusiveMaximum(inner) => inner.into_flat_at(pointer),
            crate::validation::Error::MultipleOf(inner) => inner.into_flat_at(pointer),
            crate::validation::Error::MinLength(inner) => inner.into_flat_at(pointer),
            crate::validation::Error::MaxLength(inner) => inner.into_flat_at(pointer),
            crate::validation::Error::Pattern(inner) => inner.into_flat_at(pointer),
            crate::validation::Error::MinItems(inner) => inner.into_flat_at(pointer),
            crate::validation::Error::MaxItems(inner) => inner.into_flat_at(pointer),
            crate::validation::Error::UniqueItems(inner) => inner.into_flat_at(pointer),
            crate::validation::Error::MinProperties(inner) => inner.into_flat_at(pointer),
            crate::validation::Error::MaxProperties(inner) => inner.into_flat_at(pointer),
            crate::validation::Error::Enumerate(inner) => inner.into_flat_at(pointer),
            crate::validation::Error::Items(inner) => inner.into_flat_at(pointer),
            crate::validation::Error::Properties(inner) => inner.into_flat_at(pointer),
            crate::validation::Error::Custom(inner) => {
                FlatErrors::new(vec![FlatError::new(pointer.to_owned(), inner)])
            }
        }
    }
}

impl IntoFlat for Vec<crate::validation::Error> {
    fn into_flat_at(self, pointer: &JSONPointer) -> FlatErrors {
        FlatErrors::new(self.into_iter().fold(vec![], |pre, error| {
            pre.into_iter()
                .chain(error.into_flat_at(pointer))
                .collect::<Vec<_>>()
        }))
    }
}

impl IntoFlat for ItemErrorsMap {
    fn into_flat_at(self, pointer: &JSONPointer) -> FlatErrors {
        FlatErrors::new(self.into_iter().fold(vec![], |pre, (index, errors)| {
            let parrent_pointer = merge_childs(pointer.clone(), [PathChunk::Index(index)]);
            pre.into_iter()
                .chain(
                    errors
                        .into_flat()
                        .into_iter()
                        .map(|e| e.merge_childs(parrent_pointer.clone())),
                )
                .collect::<Vec<_>>()
        }))
    }
}

impl IntoFlat for PropertyErrorsMap {
    fn into_flat_at(self, pointer: &JSONPointer) -> FlatErrors {
        FlatErrors::new(self.into_iter().fold(vec![], |pre, (property, errs)| {
            let parrent_pointer = merge_childs(
                pointer.clone(),
                [PathChunk::Property(property.to_string().into_boxed_str())],
            );
            pre.into_iter()
                .chain(
                    errs.into_flat()
                        .into_iter()
                        .map(|e| e.merge_childs(parrent_pointer.clone())),
                )
                .collect::<Vec<_>>()
        }))
    }
}

impl<T> IntoFlat for Message<T>
where
    T: ToDefaultMessage,
{
    fn into_flat_at(self, pointer: &JSONPointer) -> FlatErrors {
        FlatErrors::new(vec![FlatError::new(
            pointer.to_owned(),
            self.error().to_default_message(),
        )])
    }
}

impl IntoFlat for ArrayErrors {
    fn into_flat_at(self, pointer: &JSONPointer) -> FlatErrors {
        FlatErrors::new(
            self.errors
                .into_flat_at(pointer)
                .into_iter()
                .chain(self.items.into_flat_at(pointer))
                .collect::<Vec<_>>(),
        )
    }
}

impl IntoFlat for ObjectErrors {
    fn into_flat_at(self, pointer: &JSONPointer) -> FlatErrors {
        FlatErrors::new(
            self.errors
                .into_flat_at(pointer)
                .into_iter()
                .chain(self.properties.into_flat_at(pointer))
                .collect::<Vec<_>>(),
        )
    }
}

impl FlatError {
    pub fn new(pointer: JSONPointer, message: String) -> Self {
        Self { pointer, message }
    }

    pub fn merge_childs(self, pointer: JSONPointer) -> Self {
        Self::new(
            merge_childs(pointer, self.pointer.into_iter()),
            self.message,
        )
    }
}

impl From<Vec<FlatError>> for FlatErrors {
    fn from(errors: Vec<FlatError>) -> Self {
        Self::new(errors)
    }
}

fn merge_childs(pointer: JSONPointer, chunks: impl IntoIterator<Item = PathChunk>) -> JSONPointer {
    JSONPointer::from(
        pointer
            .into_iter()
            .chain(chunks)
            .collect::<Vec<_>>()
            .as_slice(),
    )
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
