use jsonschema::paths::{JSONPointer, PathChunk};

use crate::error::ToDefaultMessage;

use super::{ItemErrorsMap, Message, PropertyErrorsMap};

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

pub struct FlatError {
    pointer: JSONPointer,
    message: String,
}

impl FlatError {
    fn merge_childs(self, pointer: JSONPointer) -> Self {
        Self {
            pointer: merge_childs(pointer, self.pointer.into_iter()),
            message: self.message,
        }
    }
}

trait IntoFlatErrors
where
    Self: Sized,
{
    fn into_flat_errors(self) -> FlatErrors {
        self.into_flat_errors_at(&JSONPointer::default())
    }

    fn into_flat_errors_at(self, pointer: &JSONPointer) -> FlatErrors;
}

impl IntoFlatErrors for crate::validation::Errors {
    fn into_flat_errors_at(self, _pointer: &JSONPointer) -> FlatErrors {
        unimplemented!()
        // let errors = match self {
        //     crate::validation::Errors::Array(array) => array.errors.into_iter().map(|error| error),
        // };
        // FlatErrors(errors)
    }
}

impl IntoFlatErrors for crate::validation::Error {
    fn into_flat_errors_at(self, pointer: &JSONPointer) -> FlatErrors {
        FlatErrors(into_flat(pointer.to_owned(), self))
    }
}

impl IntoFlatErrors for Vec<crate::validation::Error> {
    fn into_flat_errors_at(self, pointer: &JSONPointer) -> FlatErrors {
        FlatErrors(self.into_iter().fold(vec![], |pre, e| {
            pre.into_iter()
                .chain(into_flat(pointer.clone(), e))
                .collect::<Vec<_>>()
        }))
    }
}

impl IntoFlatErrors for ItemErrorsMap {
    fn into_flat_errors_at(self, pointer: &JSONPointer) -> FlatErrors {
        FlatErrors(self.into_iter().fold(vec![], |pre, (index, errs)| {
            let parrent_pointer = merge_childs(pointer.clone(), [PathChunk::Index(index)]);
            pre.into_iter()
                .chain(
                    errs.into_flat_errors()
                        .into_iter()
                        .map(|e| e.merge_childs(parrent_pointer.clone())),
                )
                .collect::<Vec<_>>()
        }))
    }
}

impl IntoFlatErrors for PropertyErrorsMap {
    fn into_flat_errors_at(self, pointer: &JSONPointer) -> FlatErrors {
        FlatErrors(self.into_iter().fold(vec![], |pre, (property, errs)| {
            let parrent_pointer = merge_childs(
                pointer.clone(),
                [PathChunk::Property(property.to_string().into_boxed_str())],
            );
            pre.into_iter()
                .chain(
                    errs.into_flat_errors()
                        .into_iter()
                        .map(|e| e.merge_childs(parrent_pointer.clone())),
                )
                .collect::<Vec<_>>()
        }))
    }
}

fn into_flat(
    pointer: jsonschema::paths::JSONPointer,
    error: crate::validation::Error,
) -> Vec<FlatError> {
    match error {
        crate::validation::Error::Minimum(message) => single_error(pointer, message),
        crate::validation::Error::Maximum(message) => single_error(pointer, message),
        crate::validation::Error::ExclusiveMinimum(message) => single_error(pointer, message),
        crate::validation::Error::ExclusiveMaximum(message) => single_error(pointer, message),
        crate::validation::Error::MultipleOf(message) => single_error(pointer, message),
        crate::validation::Error::MinLength(message) => single_error(pointer, message),
        crate::validation::Error::MaxLength(message) => single_error(pointer, message),
        crate::validation::Error::Pattern(message) => single_error(pointer, message),
        crate::validation::Error::MinItems(message) => single_error(pointer, message),
        crate::validation::Error::MaxItems(message) => single_error(pointer, message),
        crate::validation::Error::UniqueItems(message) => single_error(pointer, message),
        crate::validation::Error::MinProperties(message) => single_error(pointer, message),
        crate::validation::Error::MaxProperties(message) => single_error(pointer, message),
        crate::validation::Error::Enumerate(message) => single_error(pointer, message),
        crate::validation::Error::Items(err) => err
            .errors
            .into_flat_errors_at(&pointer)
            .into_iter()
            .chain(err.items.into_flat_errors_at(&pointer))
            .collect(),
        crate::validation::Error::Properties(err) => err
            .errors
            .into_flat_errors_at(&pointer)
            .into_iter()
            .chain(err.properties.into_flat_errors_at(&pointer))
            .collect(),
        crate::validation::Error::Custom(message) => vec![FlatError { pointer, message }],
    }
}

fn single_error<T>(pointer: JSONPointer, message: Message<T>) -> Vec<FlatError>
where
    T: ToDefaultMessage,
{
    vec![FlatError {
        pointer,
        message: message.to_string(),
    }]
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
