mod array;
mod error;
mod generic;
mod numeric;
mod object;
mod string;

pub use array::{ValidateMaxItems, ValidateMinItems, ValidateUniqueItems};
pub use error::{
    ArrayErrors, Composited, ConvertIntoError, Error, Errors, MapErrors, ObjectErrors, VecErrors,
};
pub use generic::{ValidateCompositedEnumerate, ValidateEnumerate};
pub use numeric::{
    ValidateCompositedExclusiveMaximum, ValidateCompositedExclusiveMinimum,
    ValidateCompositedMaximum, ValidateCompositedMinimum, ValidateCompositedMultipleOf,
    ValidateExclusiveMaximum, ValidateExclusiveMinimum, ValidateMaximum, ValidateMinimum,
    ValidateMultipleOf,
};
pub use object::{ValidateMaxProperties, ValidateMinProperties};
pub use string::{ValidateMaxLength, ValidateMinLength, ValidatePattern};

use crate::{
    MaxLengthErrorParams, MaxPropertiesErrorParams, MinLengthErrorParams, MinPropertiesErrorParams,
    PatternErrorParams,
};

macro_rules! impl_composited_validation1 {
    ($CompositedValidateTrait:ident, $ValidateTrait:ident, $ErrorParams:tt, $composited_validation_method:ident, $validation_method:ident, $limit_type:ty) => {
        pub trait $CompositedValidateTrait {
            fn $composited_validation_method(
                &self,
                limit: $limit_type,
            ) -> Result<(), crate::validation::Composited<$ErrorParams>>;
        }

        impl<T> $CompositedValidateTrait for T
        where
            T: $ValidateTrait,
        {
            fn $composited_validation_method(
                &self,
                limit: $limit_type,
            ) -> Result<(), crate::validation::Composited<$ErrorParams>> {
                self.$validation_method(limit)
                    .map_err(|error| crate::validation::Composited::Single(error))
            }
        }

        impl<T> $CompositedValidateTrait for Vec<T>
        where
            T: $CompositedValidateTrait,
        {
            fn $composited_validation_method(
                &self,
                limit: $limit_type,
            ) -> Result<(), crate::validation::Composited<$ErrorParams>> {
                let mut errors = vec![];
                self.iter().for_each(|item| {
                    item.$composited_validation_method(limit)
                        .map_err(|error| errors.push(error))
                        .ok();
                });

                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(crate::validation::Composited::Array(errors))
                }
            }
        }

        impl<T, const N: usize> $CompositedValidateTrait for [T; N]
        where
            T: $CompositedValidateTrait,
        {
            fn $composited_validation_method(
                &self,
                limit: $limit_type,
            ) -> Result<(), crate::validation::Composited<$ErrorParams>> {
                let mut errors = vec![];
                self.iter().for_each(|item| {
                    item.$composited_validation_method(limit)
                        .map_err(|error| errors.push(error))
                        .ok();
                });

                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(crate::validation::Composited::Array(errors))
                }
            }
        }

        impl<T> $CompositedValidateTrait for Option<T>
        where
            T: $CompositedValidateTrait,
        {
            fn $composited_validation_method(
                &self,
                limit: $limit_type,
            ) -> Result<(), crate::validation::Composited<$ErrorParams>> {
                match self {
                    Some(value) => value.$composited_validation_method(limit),
                    None => Ok(()),
                }
            }
        }
    };
}

// String
impl_composited_validation1!(
    ValidateCompositedMaxLength,
    ValidateMaxLength,
    MaxLengthErrorParams,
    validate_composited_max_length,
    validate_max_length,
    usize
);
impl_composited_validation1!(
    ValidateCompositedMinLength,
    ValidateMinLength,
    MinLengthErrorParams,
    validate_composited_min_length,
    validate_min_length,
    usize
);
impl_composited_validation1!(
    ValidateCompositedPattern,
    ValidatePattern,
    PatternErrorParams,
    validate_composited_pattern,
    validate_pattern,
    &regex::Regex
);

// Object
impl_composited_validation1!(
    ValidateCompositedMaxProperties,
    ValidateMaxProperties,
    MaxPropertiesErrorParams,
    validate_composited_max_properties,
    validate_max_properties,
    usize
);
impl_composited_validation1!(
    ValidateCompositedMinProperties,
    ValidateMinProperties,
    MinPropertiesErrorParams,
    validate_composited_min_properties,
    validate_min_properties,
    usize
);
