mod flat_error;
mod flat_errors;
mod into_flat;

pub use flat_error::FlatError;
pub use flat_errors::FlatErrors;
pub use into_flat::IntoFlat;
use jsonschema::paths::{JSONPointer, PathChunk};

pub(crate) fn merge_childs(
    pointer: JSONPointer,
    chunks: impl IntoIterator<Item = PathChunk>,
) -> JSONPointer {
    JSONPointer::from(
        pointer
            .into_iter()
            .chain(chunks)
            .collect::<Vec<_>>()
            .as_slice(),
    )
}
