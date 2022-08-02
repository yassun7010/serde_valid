use crate::error::ToDefaultMessage;

use super::Error;
use crate::error::{
    EnumerateErrorParams, ExclusiveMaximumErrorParams, ExclusiveMinimumErrorParams,
    MaxItemsErrorParams, MaxLengthErrorParams, MaxPropertiesErrorParams, MaximumErrorParams,
    MinItemsErrorParams, MinLengthErrorParams, MinPropertiesErrorParams, MinimumErrorParams,
    MultipleOfErrorParams, PatternErrorParams, UniqueItemsErrorParams,
};
use indexmap::IndexMap;

#[derive(Debug)]
pub enum Composited<ErrorParams> {
    Single(ErrorParams),
    Array(Vec<Composited<ErrorParams>>),
}

pub trait IntoError<Params>: Sized
where
    Params: ToDefaultMessage,
{
    fn into_error(self) -> crate::validation::Error {
        self.into_error_by(Params::to_default_message)
    }

    fn into_error_by(self, format_fn: fn(&Params) -> String) -> crate::validation::Error;
}

macro_rules! impl_into_error {
    ($ErrorType:ident) => {
        paste::paste! {
            impl IntoError<[<$ErrorType ErrorParams>]> for Composited<[<$ErrorType ErrorParams>]> {
                fn into_error_by(self, format_fn: fn(&[<$ErrorType ErrorParams>]) -> String) -> Error {
                    match self {
                        Composited::Single(single) => Error::$ErrorType(crate::error::Message::new(single, format_fn)),
                        Composited::Array(array) => Error::Items(crate::validation::ArrayErrors::new(
                            Vec::with_capacity(0),
                            array
                                .into_iter()
                                .enumerate()
                                .map(|(index, params)| {
                                    (index, crate::validation::Errors::NewType(vec![params.into_error_by(format_fn)]))
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
