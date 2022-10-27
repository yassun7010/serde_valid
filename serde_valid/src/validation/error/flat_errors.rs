use jsonschema::paths::{JSONPointer, PathChunk};

use crate::error::ToDefaultMessage;

use super::{ArrayErrors, ItemErrorsMap, Message, ObjectErrors, PropertyErrorsMap};

pub struct FlatError {
    pointer: JSONPointer,
    message: String,
}

impl FlatError {
    pub fn new(pointer: JSONPointer, message: String) -> Self {
        Self { pointer, message }
    }

    pub fn merge_childs(self, pointer: JSONPointer) -> Self {
        Self {
            pointer: merge_childs(pointer, self.pointer.into_iter()),
            message: self.message,
        }
    }
}

pub struct FlatErrors(Vec<FlatError>);

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

impl From<Vec<FlatError>> for FlatErrors {
    fn from(errors: Vec<FlatError>) -> Self {
        Self(errors)
    }
}

trait IntoFlat
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
            crate::validation::Error::Custom(inner) => inner.into_flat_at(pointer),
        }
    }
}

impl IntoFlat for Vec<crate::validation::Error> {
    fn into_flat_at(self, pointer: &JSONPointer) -> FlatErrors {
        FlatErrors(self.into_iter().fold(vec![], |pre, error| {
            pre.into_iter()
                .chain(error.into_flat_at(pointer))
                .collect::<Vec<_>>()
        }))
    }
}

impl IntoFlat for ItemErrorsMap {
    fn into_flat_at(self, pointer: &JSONPointer) -> FlatErrors {
        FlatErrors(self.into_iter().fold(vec![], |pre, (index, errors)| {
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
        FlatErrors(self.into_iter().fold(vec![], |pre, (property, errs)| {
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
        FlatErrors(vec![FlatError {
            pointer: pointer.to_owned(),
            message: self.to_string(),
        }])
    }
}

impl IntoFlat for ArrayErrors {
    fn into_flat_at(self, pointer: &JSONPointer) -> FlatErrors {
        FlatErrors(
            self.errors
                .into_flat_at(pointer)
                .into_iter()
                .chain(self.items.into_flat_at(pointer))
                .collect(),
        )
    }
}

impl IntoFlat for ObjectErrors {
    fn into_flat_at(self, pointer: &JSONPointer) -> FlatErrors {
        FlatErrors(
            self.errors
                .into_flat_at(pointer)
                .into_iter()
                .chain(self.properties.into_flat_at(pointer))
                .collect(),
        )
    }
}

impl IntoFlat for String {
    fn into_flat_at(self, pointer: &JSONPointer) -> FlatErrors {
        FlatErrors(vec![FlatError {
            pointer: pointer.to_owned(),
            message: self,
        }])
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
