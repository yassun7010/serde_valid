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
    ValidateMultipleOf,
};
pub use object::{ValidateMaxProperties, ValidateMinProperties};
pub use string::{ValidateMaxLength, ValidateMinLength, ValidatePattern};

use crate::{MaxLengthErrorParams, MinLengthErrorParams, PatternErrorParams};

#[derive(Debug)]
pub enum Multiple<ErrorParams> {
    Single(ErrorParams),
    Array(Vec<Multiple<ErrorParams>>),
}

macro_rules! impl_multi_validation1 {
    ($ValidateTrait:ident, $ErrorParams:tt, $validation_method:ident, $limit_type:ty) => {
        paste::item! {
            pub trait [< $ValidateTrait s >] {
                fn [< $validation_method s >](
                    &self,
                    limit: $limit_type,
                ) -> Result<(), crate::validation::Multiple<$ErrorParams>>;
            }

            impl<T> [< $ValidateTrait s >] for T
            where
                T: $ValidateTrait,
            {
                fn [< $validation_method s >](
                    &self,
                    limit: $limit_type,
                ) -> Result<(), crate::validation::Multiple<$ErrorParams>> {
                    self.$validation_method(limit)
                        .map_err(|error| crate::validation::Multiple::Single(error))
                }
            }

            impl<T> [< $ValidateTrait s >] for Vec<T>
            where
                T: [< $ValidateTrait s >],
            {
                fn [< $validation_method s >](
                    &self,
                    limit: $limit_type,
                ) -> Result<(), crate::validation::Multiple<$ErrorParams>> {
                    let mut errors = vec![];
                    self.iter().for_each(|item| {
                        item.[< $validation_method s >](limit)
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

            impl<T, const N: usize> [< $ValidateTrait s >] for [T; N]
            where
                T: [< $ValidateTrait s >],
            {
                fn [< $validation_method s >](
                    &self,
                    limit: $limit_type,
                ) -> Result<(), crate::validation::Multiple<$ErrorParams>> {
                    let mut errors = vec![];
                    self.iter().for_each(|item| {
                        item.[< $validation_method s >](limit)
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

            impl<T> [< $ValidateTrait s >] for Option<T>
            where
                T: [< $ValidateTrait s >],
            {
                fn [< $validation_method s >](
                    &self,
                    limit: $limit_type,
                ) -> Result<(), crate::validation::Multiple<$ErrorParams>> {
                    match self {
                        Some(value) => value.[< $validation_method s >](limit),
                        None => Ok(()),
                    }
                }
            }
        }
    };
}

impl_multi_validation1!(
    ValidateMaxLength,
    MaxLengthErrorParams,
    validate_max_length,
    usize
);
impl_multi_validation1!(
    ValidateMinLength,
    MinLengthErrorParams,
    validate_min_length,
    usize
);
impl_multi_validation1!(
    ValidatePattern,
    PatternErrorParams,
    validate_pattern,
    &regex::Regex
);
