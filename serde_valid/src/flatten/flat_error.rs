use jsonschema::paths::{JSONPointer, PathChunk};
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct FlatError {
    pub message: String,
    pub path: JSONPointer,
}

impl FlatError {
    pub fn new(message: String, path: JSONPointer) -> Self {
        Self { message, path }
    }

    pub fn merge_childs(self, path: JSONPointer) -> Self {
        Self::new(self.message, merge_childs(path, self.path.into_iter()))
    }
}

pub(crate) fn merge_childs(
    path: JSONPointer,
    chunks: impl IntoIterator<Item = PathChunk>,
) -> JSONPointer {
    JSONPointer::from(
        path.into_iter()
            .chain(chunks)
            .collect::<Vec<_>>()
            .as_slice(),
    )
}
