use jsonschema::paths::{JSONPointer, PathChunk};

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
    fn embed(self, pointer: JSONPointer) -> Self {
        Self {
            pointer: merge(pointer, self.pointer.into_iter()),
            message: self.message,
        }
    }
}

trait IntoFlatErrors {
    fn into_flat_errors(self) -> FlatErrors;
}

impl IntoFlatErrors for crate::validation::Errors {
    fn into_flat_errors(self) -> FlatErrors {
        unimplemented!()
        // let errors = match self {
        //     crate::validation::Errors::Array(array) => array.errors.into_iter().map(|error| error),
        // };
        // FlatErrors(errors)
    }
}

impl IntoFlatErrors for crate::validation::Error {
    fn into_flat_errors(self) -> FlatErrors {
        FlatErrors(into_flat(JSONPointer::default(), self))
    }
}

fn into_flat(
    pointer: jsonschema::paths::JSONPointer,
    error: crate::validation::Error,
) -> Vec<FlatError> {
    match error {
        crate::validation::Error::Items(err) => err
            .errors
            .into_iter()
            .fold(vec![], |pre, e| {
                pre.into_iter()
                    .chain(into_flat(pointer.clone(), e))
                    .collect::<Vec<_>>()
            })
            .into_iter()
            .chain(err.items.into_iter().fold(vec![], |pre, (index, errs)| {
                let parrent_pointer = merge(pointer.clone(), [PathChunk::Index(index)]);
                pre.into_iter()
                    .chain(
                        errs.into_flat_errors()
                            .into_iter()
                            .map(|e| e.embed(parrent_pointer.clone())),
                    )
                    .collect::<Vec<_>>()
            }))
            .collect(),
        crate::validation::Error::Properties(err) => err
            .errors
            .into_iter()
            .fold(vec![], |pre, e| {
                pre.into_iter()
                    .chain(into_flat(pointer.clone(), e))
                    .collect::<Vec<_>>()
            })
            .into_iter()
            .chain(
                err.properties
                    .into_iter()
                    .fold(vec![], |pre, (property, errs)| {
                        let parrent_pointer = merge(
                            pointer.clone(),
                            [PathChunk::Property(property.to_string().into_boxed_str())],
                        );

                        pre.into_iter()
                            .chain(
                                errs.into_flat_errors()
                                    .into_iter()
                                    .map(|e| e.embed(parrent_pointer.clone())),
                            )
                            .collect::<Vec<_>>()
                    }),
            )
            .collect(),
    }
}

fn merge(pointer: JSONPointer, chunks: impl IntoIterator<Item = PathChunk>) -> JSONPointer {
    JSONPointer::from(
        pointer
            .into_iter()
            .chain(chunks)
            .collect::<Vec<_>>()
            .as_slice(),
    )
}
