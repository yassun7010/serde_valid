use jsonschema::paths::JSONPointer;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct FlatError {
    pub pointer: JSONPointer,
    pub message: String,
}
