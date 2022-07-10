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

pub trait IntoError {
    type Params;
    fn into_error(self, format_fn: fn(&Self::Params) -> String) -> Error;
}

impl IntoError for Composited<EnumerateErrorParams> {
    type Params = EnumerateErrorParams;
    fn into_error(self, format_fn: fn(&Self::Params) -> String) -> Error {
        match self {
            Composited::Single(single) => Error::Enumerate(Message::new(single, format_fn)),
            Composited::Array(array) => Error::Items(ArrayErrors::new(
                Vec::with_capacity(0),
                array
                    .into_iter()
                    .enumerate()
                    .map(|(index, params)| {
                        (index, Errors::NewType(vec![params.into_error(format_fn)]))
                    })
                    .collect::<IndexMap<_, _>>(),
            )),
        }
    }
}
