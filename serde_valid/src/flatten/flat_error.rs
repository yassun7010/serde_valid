use jsonschema::paths::JSONPointer;

#[derive(Debug, PartialEq, Eq)]
pub struct FlatError {
    pub pointer: JSONPointer,
    pub message: String,
}
