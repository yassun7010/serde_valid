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
use paste::paste;

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

macro_rules! impl_into_error {
    ($ErrorType:ident) => {
        paste! {
            impl IntoError for Composited<[<$ErrorType ErrorParams>]> {
                type Params = [<$ErrorType ErrorParams>];
                fn into_error(self, format_fn: fn(&Self::Params) -> String) -> Error {
                    match self {
                        Composited::Single(single) => Error::$ErrorType(Message::new(single, format_fn)),
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
        }
    };
}

// Global
impl_into_error!(Enumerate);

// Numeric
impl_into_error!(Maximum);
impl_into_error!(Minimum);
impl_into_error!(ExclusiveMaximum);
impl_into_error!(ExclusiveMinimum);
