mod array_erros;
mod error;
mod errors;
mod object_errors;

pub use crate::error::{
    EnumerateErrorParams, ExclusiveMaximumErrorParams, ExclusiveMinimumErrorParams,
    MaxItemsErrorParams, MaxLengthErrorParams, MaxPropertiesErrorParams, MaximumErrorParams,
    Message, MinItemsErrorParams, MinLengthErrorParams, MinPropertiesErrorParams,
    MinimumErrorParams, MultipleOfErrorParams, PatternErrorParams, UniqueItemsErrorParams,
};
pub use array_erros::ArrayErrors;
pub use error::Error;
pub use errors::Errors;
use indexmap::IndexMap;
pub use object_errors::ObjectErrors;

pub type VecErrors = Vec<Error>;
pub type MapErrors = IndexMap<&'static str, VecErrors>;

#[derive(Debug)]
pub enum Composited<ErrorParams> {
    Single(ErrorParams),
    Array(Vec<Composited<ErrorParams>>),
}
