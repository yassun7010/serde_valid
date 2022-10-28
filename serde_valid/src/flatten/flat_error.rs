use jsonschema::paths::JSONPointer;

use super::merge_childs;

#[derive(Debug, PartialEq, Eq)]
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
