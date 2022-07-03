mod max_properties;
mod min_properties;

pub use max_properties::ValidateMaxProperties;
pub use min_properties::ValidateMinProperties;

use crate::{MaxPropertiesErrorParams, MinPropertiesErrorParams};

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

impl_validate_object_size!(
    ValidateMaxProperties,
    MaxPropertiesErrorParams,
    validate_max_properties
);
impl_validate_object_size!(
    ValidateMinProperties,
    MinPropertiesErrorParams,
    validate_min_properties
);
