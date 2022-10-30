use jsonschema::paths::{JSONPointer, PathChunk};
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct FlatError {
    pub path: JSONPointer,
    pub message: String,
}

impl FlatError {
    pub fn new(path: impl Into<JSONPointer>, message: String) -> Self {
        Self {
            message,
            path: path.into(),
        }
    }

    pub fn merge_childs(self, path: impl IntoIterator<Item = PathChunk>) -> Self {
        Self::new(
            JSONPointer::from(
                path.into_iter()
                    .chain(self.path.into_iter())
                    .collect::<Vec<_>>()
                    .as_slice(),
            ),
            self.message,
        )
    }
}
