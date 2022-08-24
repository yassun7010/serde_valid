mod array_erros;
mod composited;
mod errors;
mod object_errors;

use crate::error::ToDefaultMessage;
pub use crate::error::{
    EnumerateError, ExclusiveMaximumError, ExclusiveMinimumError, MaxItemsError, MaxLengthError,
    MaxPropertiesError, MaximumError, Message, MinItemsError, MinLengthError, MinPropertiesError,
    MinimumError, MultipleOfError, PatternError, UniqueItemsError,
};
pub use array_erros::ArrayErrors;
pub use composited::{Composited, IntoError};
pub use errors::Errors;
use indexmap::IndexMap;
pub use object_errors::ObjectErrors;

pub type VecErrors = Vec<Error>;
pub type ItemErrorsMap = IndexMap<usize, Errors>;
pub type ItemVecErrorsMap = IndexMap<usize, VecErrors>;
pub type PropertyErrorsMap = IndexMap<&'static str, Errors>;
pub type PropertyVecErrorsMap = IndexMap<&'static str, VecErrors>;

#[derive(Debug)]
pub enum Composited<Error> {
    Single(Error),
    Array(Vec<Composited<Error>>),
}

pub trait IntoError<Params>: Sized
where
    Params: ToDefaultMessage,
{
    fn into_error(self) -> Error {
        self.into_error_by(Params::to_default_message)
    }

    fn into_error_by(self, format_fn: fn(&Params) -> String) -> Error;
}

macro_rules! impl_into_error {
    ($ErrorType:ident) => {
        paste::paste! {
            impl IntoError<[<$ErrorType Error>]> for Composited<[<$ErrorType Error>]> {
                fn into_error_by(self, format_fn: fn(&[<$ErrorType Error>]) -> String) -> Error {
                    match self {
                        Composited::Single(single) => Error::$ErrorType(Message::new(single, format_fn)),
                        Composited::Array(array) => Error::Items(ArrayErrors::new(
                            Vec::with_capacity(0),
                            array
                                .into_iter()
                                .enumerate()
                                .map(|(index, params)| {
                                    (index, Errors::NewType(vec![params.into_error_by(format_fn)]))
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
impl_into_error!(MultipleOf);

// String
impl_into_error!(MaxLength);
impl_into_error!(MinLength);
impl_into_error!(Pattern);

// Array
impl_into_error!(MaxItems);
impl_into_error!(MinItems);
impl_into_error!(UniqueItems);

// Object
impl_into_error!(MaxProperties);
impl_into_error!(MinProperties);
