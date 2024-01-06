use jsonschema::paths::{JSONPointer, PathChunk};
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct FlatError {
    pub error: String,
    pub instance_location: JSONPointer,
}

impl FlatError {
    pub fn new(instance_location: impl Into<JSONPointer>, error: String) -> Self {
        Self {
            error,
            instance_location: instance_location.into(),
        }
    }

    pub fn merge_childs(self, instance_location: impl IntoIterator<Item = PathChunk>) -> Self {
        Self::new(
            JSONPointer::from(
                instance_location
                    .into_iter()
                    .chain(self.instance_location)
                    .collect::<Vec<_>>()
                    .as_slice(),
            ),
            self.error,
        )
    }
}
