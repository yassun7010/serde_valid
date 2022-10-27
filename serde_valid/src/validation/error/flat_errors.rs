use jsonschema::paths::{JSONPointer, PathChunk};

use crate::error::ToDefaultMessage;

use super::{ArrayErrors, ItemErrorsMap, Message, ObjectErrors, PropertyErrorsMap};

pub struct FlattenErrors(Vec<FlattenError>);

impl IntoIterator for FlattenErrors {
    type Item = FlattenError;
    type IntoIter = <Vec<FlattenError> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a FlattenErrors {
    type Item = &'a FlattenError;
    type IntoIter = std::slice::Iter<'a, FlattenError>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl From<Vec<FlattenError>> for FlattenErrors {
    fn from(errors: Vec<FlattenError>) -> Self {
        Self(errors)
    }
}

pub struct FlattenError {
    pointer: JSONPointer,
    message: String,
}

impl FlattenError {
    fn merge_childs(self, pointer: JSONPointer) -> Self {
        Self {
            pointer: merge_childs(pointer, self.pointer.into_iter()),
            message: self.message,
        }
    }
}

trait IntoFlatten
where
    Self: Sized,
{
    fn into_flatten(self) -> FlattenErrors {
        self.into_flatten_at(&JSONPointer::default())
    }

    fn into_flatten_at(self, pointer: &JSONPointer) -> FlattenErrors;
}

impl IntoFlatten for crate::validation::Errors {
    fn into_flatten_at(self, pointer: &JSONPointer) -> FlattenErrors {
        match self {
            crate::validation::Errors::Array(errors) => errors.into_flatten_at(pointer),
            crate::validation::Errors::Object(errors) => errors.into_flatten_at(pointer),
            crate::validation::Errors::NewType(errors) => errors.into_flatten_at(pointer),
        }
    }
}

impl IntoFlatten for crate::validation::Error {
    fn into_flatten_at(self, pointer: &JSONPointer) -> FlattenErrors {
        match self {
            crate::validation::Error::Minimum(inner) => inner.into_flatten_at(pointer),
            crate::validation::Error::Maximum(inner) => inner.into_flatten_at(pointer),
            crate::validation::Error::ExclusiveMinimum(inner) => inner.into_flatten_at(pointer),
            crate::validation::Error::ExclusiveMaximum(inner) => inner.into_flatten_at(pointer),
            crate::validation::Error::MultipleOf(inner) => inner.into_flatten_at(pointer),
            crate::validation::Error::MinLength(inner) => inner.into_flatten_at(pointer),
            crate::validation::Error::MaxLength(inner) => inner.into_flatten_at(pointer),
            crate::validation::Error::Pattern(inner) => inner.into_flatten_at(pointer),
            crate::validation::Error::MinItems(inner) => inner.into_flatten_at(pointer),
            crate::validation::Error::MaxItems(inner) => inner.into_flatten_at(pointer),
            crate::validation::Error::UniqueItems(inner) => inner.into_flatten_at(pointer),
            crate::validation::Error::MinProperties(inner) => inner.into_flatten_at(pointer),
            crate::validation::Error::MaxProperties(inner) => inner.into_flatten_at(pointer),
            crate::validation::Error::Enumerate(inner) => inner.into_flatten_at(pointer),
            crate::validation::Error::Items(inner) => inner.into_flatten_at(pointer),
            crate::validation::Error::Properties(inner) => inner.into_flatten_at(pointer),
            crate::validation::Error::Custom(inner) => inner.into_flatten_at(pointer),
        }
    }
}

impl IntoFlatten for Vec<crate::validation::Error> {
    fn into_flatten_at(self, pointer: &JSONPointer) -> FlattenErrors {
        FlattenErrors(self.into_iter().fold(vec![], |pre, error| {
            pre.into_iter()
                .chain(error.into_flatten_at(pointer))
                .collect::<Vec<_>>()
        }))
    }
}

impl IntoFlatten for ItemErrorsMap {
    fn into_flatten_at(self, pointer: &JSONPointer) -> FlattenErrors {
        FlattenErrors(self.into_iter().fold(vec![], |pre, (index, errors)| {
            let parrent_pointer = merge_childs(pointer.clone(), [PathChunk::Index(index)]);
            pre.into_iter()
                .chain(
                    errors
                        .into_flatten()
                        .into_iter()
                        .map(|e| e.merge_childs(parrent_pointer.clone())),
                )
                .collect::<Vec<_>>()
        }))
    }
}

impl IntoFlatten for PropertyErrorsMap {
    fn into_flatten_at(self, pointer: &JSONPointer) -> FlattenErrors {
        FlattenErrors(self.into_iter().fold(vec![], |pre, (property, errs)| {
            let parrent_pointer = merge_childs(
                pointer.clone(),
                [PathChunk::Property(property.to_string().into_boxed_str())],
            );
            pre.into_iter()
                .chain(
                    errs.into_flatten()
                        .into_iter()
                        .map(|e| e.merge_childs(parrent_pointer.clone())),
                )
                .collect::<Vec<_>>()
        }))
    }
}

impl<T> IntoFlatten for Message<T>
where
    T: ToDefaultMessage,
{
    fn into_flatten_at(self, pointer: &JSONPointer) -> FlattenErrors {
        FlattenErrors(vec![FlattenError {
            pointer: pointer.to_owned(),
            message: self.to_string(),
        }])
    }
}

impl IntoFlatten for ArrayErrors {
    fn into_flatten_at(self, pointer: &JSONPointer) -> FlattenErrors {
        FlattenErrors(
            self.errors
                .into_flatten_at(pointer)
                .into_iter()
                .chain(self.items.into_flatten_at(pointer))
                .collect(),
        )
    }
}

impl IntoFlatten for ObjectErrors {
    fn into_flatten_at(self, pointer: &JSONPointer) -> FlattenErrors {
        FlattenErrors(
            self.errors
                .into_flatten_at(pointer)
                .into_iter()
                .chain(self.properties.into_flatten_at(pointer))
                .collect(),
        )
    }
}

impl IntoFlatten for String {
    fn into_flatten_at(self, pointer: &JSONPointer) -> FlattenErrors {
        FlattenErrors(vec![FlattenError {
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
