mod array;
mod error;
mod generic;
mod numeric;
mod object;
mod string;

pub use array::{ValidateMaxItems, ValidateMinItems, ValidateUniqueItems};
pub use error::{ArrayErrors, Error, Errors, MapErrors, ObjectErrors, VecErrors};
pub use generic::ValidateEnumerate;
pub use numeric::{
    ValidateExclusiveMaximum, ValidateExclusiveMinimum, ValidateMaximum, ValidateMinimum,
    ValidateMultiExclusiveMaximum, ValidateMultiExclusiveMinimum, ValidateMultiMaximum,
    ValidateMultiMinimum, ValidateMultipleOf,
};
pub use object::{ValidateMaxProperties, ValidateMinProperties};
pub use string::{ValidateMaxLength, ValidateMinLength, ValidatePattern};

use crate::{
    MaxLengthErrorParams, MaxPropertiesErrorParams, MinLengthErrorParams, MinPropertiesErrorParams,
    PatternErrorParams,
};

#[derive(Debug)]
pub enum Multiple<ErrorParams> {
    Single(ErrorParams),
    Array(Vec<Multiple<ErrorParams>>),
}

macro_rules! impl_multi_validation1 {
    ($MultiValidateTrait:ident, $ValidateTrait:ident, $ErrorParams:tt, $multi_validation_method:ident, $validation_method:ident, $limit_type:ty) => {
        pub trait $MultiValidateTrait {
            fn $multi_validation_method(
                &self,
                limit: $limit_type,
            ) -> Result<(), crate::validation::Multiple<$ErrorParams>>;
        }

        impl<T> $MultiValidateTrait for T
        where
            T: $ValidateTrait,
        {
            fn $multi_validation_method(
                &self,
                limit: $limit_type,
            ) -> Result<(), crate::validation::Multiple<$ErrorParams>> {
                self.$validation_method(limit)
                    .map_err(|error| crate::validation::Multiple::Single(error))
            }
        }

        impl<T> $MultiValidateTrait for Vec<T>
        where
            T: $MultiValidateTrait,
        {
            fn $multi_validation_method(
                &self,
                limit: $limit_type,
            ) -> Result<(), crate::validation::Multiple<$ErrorParams>> {
                let mut errors = vec![];
                self.iter().for_each(|item| {
                    item.$multi_validation_method(limit)
                        .map_err(|error| errors.push(error))
                        .ok();
                });

                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(crate::validation::Multiple::Array(errors))
                }
            }
        }

        impl<T, const N: usize> $MultiValidateTrait for [T; N]
        where
            T: $MultiValidateTrait,
        {
            fn $multi_validation_method(
                &self,
                limit: $limit_type,
            ) -> Result<(), crate::validation::Multiple<$ErrorParams>> {
                let mut errors = vec![];
                self.iter().for_each(|item| {
                    item.$multi_validation_method(limit)
                        .map_err(|error| errors.push(error))
                        .ok();
                });

                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(crate::validation::Multiple::Array(errors))
                }
            }
        }

        impl<T> $MultiValidateTrait for Option<T>
        where
            T: $MultiValidateTrait,
        {
            fn $multi_validation_method(
                &self,
                limit: $limit_type,
            ) -> Result<(), crate::validation::Multiple<$ErrorParams>> {
                match self {
                    Some(value) => value.$multi_validation_method(limit),
                    None => Ok(()),
                }
            }
        }
    };
}

// String
impl_multi_validation1!(
    ValidateMultiMaxLength,
    ValidateMaxLength,
    MaxLengthErrorParams,
    validate_multi_max_length,
    validate_max_length,
    usize
);
impl_multi_validation1!(
    ValidateMultiMinLength,
    ValidateMinLength,
    MinLengthErrorParams,
    validate_multi_min_length,
    validate_min_length,
    usize
);
impl_multi_validation1!(
    ValidateMultiPattern,
    ValidatePattern,
    PatternErrorParams,
    validate_multi_pattern,
    validate_pattern,
    &regex::Regex
);

// Object
impl_multi_validation1!(
    ValidateMultiMaxProperties,
    ValidateMaxProperties,
    MaxPropertiesErrorParams,
    validate_multi_max_properties,
    validate_max_properties,
    usize
);
impl_multi_validation1!(
    ValidateMultiMinProperties,
    ValidateMinProperties,
    MinPropertiesErrorParams,
    validate_multi_min_properties,
    validate_min_properties,
    usize
);
