mod max_length;
mod min_length;
mod pattern;
pub use max_length::ValidateMaxLength;
pub use min_length::ValidateMinLength;
pub use pattern::ValidatePattern;

use crate::{MaxLengthErrorParams, MinLengthErrorParams};

macro_rules! impl_validate_object_size {
    ($ValidateTrait:tt, $ErrorParams:tt, $validation_method:ident) => {
        impl<T> $ValidateTrait for Vec<T>
        where
            T: $ValidateTrait,
        {
            fn $validation_method(&self, limit: usize) -> Result<(), $ErrorParams> {
                for item in self {
                    item.$validation_method(limit)?
                }

                Ok(())
            }
        }

        impl<T, const N: usize> $ValidateTrait for [T; N]
        where
            T: $ValidateTrait,
        {
            fn $validation_method(&self, limit: usize) -> Result<(), $ErrorParams> {
                for item in self {
                    item.$validation_method(limit)?
                }

                Ok(())
            }
        }

        impl<T> $ValidateTrait for Option<T>
        where
            T: $ValidateTrait,
        {
            fn $validation_method(&self, limit: usize) -> Result<(), $ErrorParams> {
                match self {
                    Some(value) => value.$validation_method(limit),
                    None => Ok(()),
                }
            }
        }
    };
}

impl_validate_object_size!(ValidateMaxLength, MaxLengthErrorParams, validate_max_length);
impl_validate_object_size!(ValidateMinLength, MinLengthErrorParams, validate_min_length);
