use jsonschema::paths::{JSONPointer, PathChunk};
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct FlatError {
    pub message: String,
    pub path: JSONPointer,
}

impl FlatError {
    pub fn new(message: String, path: impl Into<JSONPointer>) -> Self {
        Self {
            message,
            path: path.into(),
        }
    }

    pub fn merge_childs(self, path: impl IntoIterator<Item = PathChunk>) -> Self {
        Self::new(
            self.message,
            JSONPointer::from(
                path.into_iter()
                    .chain(self.path.into_iter())
                    .collect::<Vec<_>>()
                    .as_slice(),
            ),
        )
    }
}
