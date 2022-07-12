mod array;
mod error;
mod generic;
mod numeric;
mod object;
mod string;

pub use array::{ValidateMaxItems, ValidateMinItems, ValidateUniqueItems};
pub use error::{
    ArrayErrors, Composited, Error, Errors, IntoError, MapErrors, ObjectErrors, VecErrors,
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
    ($ErrorType:ident, $limit_type:ty) => {
        paste::paste! {
            pub trait [<ValidateComposited $ErrorType>] {
                fn [<validate_composited_ $ErrorType:snake>](
                    &self,
                    limit: $limit_type,
                ) -> Result<(), crate::validation::Composited<[<$ErrorType ErrorParams>]>>;
            }

            impl<T> [<ValidateComposited $ErrorType>] for T
            where
                T: [<Validate $ErrorType>],
            {
                fn [<validate_composited_ $ErrorType:snake>](
                    &self,
                    limit: $limit_type,
                ) -> Result<(), crate::validation::Composited<[<$ErrorType ErrorParams>]>> {
                    self.[<validate_ $ErrorType:snake>](limit)
                        .map_err(|error| crate::validation::Composited::Single(error))
                }
            }

            impl<T> [<ValidateComposited $ErrorType>] for Vec<T>
            where
                T: [<ValidateComposited $ErrorType>],
            {
                fn [<validate_composited_ $ErrorType:snake>](
                    &self,
                    limit: $limit_type,
                ) -> Result<(), crate::validation::Composited<[<$ErrorType ErrorParams>]>> {
                    let mut errors = vec![];
                    self.iter().for_each(|item| {
                        item.[<validate_composited_ $ErrorType:snake>](limit)
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

            impl<T, const N: usize> [<ValidateComposited $ErrorType>] for [T; N]
            where
                T: [<ValidateComposited $ErrorType>],
            {
                fn [<validate_composited_ $ErrorType:snake>](
                    &self,
                    limit: $limit_type,
                ) -> Result<(), crate::validation::Composited<[<$ErrorType ErrorParams>]>> {
                    let mut errors = vec![];
                    self.iter().for_each(|item| {
                        item.[<validate_composited_ $ErrorType:snake>](limit)
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

            impl<T> [<ValidateComposited $ErrorType>] for Option<T>
            where
                T: [<ValidateComposited $ErrorType>],
            {
                fn [<validate_composited_ $ErrorType:snake>](
                    &self,
                    limit: $limit_type,
                ) -> Result<(), crate::validation::Composited<[<$ErrorType ErrorParams>]>> {
                    match self {
                        Some(value) => value.[<validate_composited_ $ErrorType:snake>](limit),
                        None => Ok(()),
                    }
                }
            }
        }
    };
}

// String
impl_composited_validation1!(MaxLength, usize);
impl_composited_validation1!(MinLength, usize);
impl_composited_validation1!(Pattern, &regex::Regex);

// Object
impl_composited_validation1!(MaxProperties, usize);
impl_composited_validation1!(MinProperties, usize);
