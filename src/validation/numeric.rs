mod exclusive_maximum;
mod exclusive_minimum;
mod maximum;
mod minimum;
mod multiple_of;
pub use exclusive_maximum::{ValidateCompositedExclusiveMaximum, ValidateExclusiveMaximum};
pub use exclusive_minimum::{ValidateCompositedExclusiveMinimum, ValidateExclusiveMinimum};
pub use maximum::{ValidateCompositedMaximum, ValidateMaximum};
pub use minimum::{ValidateCompositedMinimum, ValidateMinimum};
pub use multiple_of::{ValidateCompositedMultipleOf, ValidateMultipleOf};

macro_rules! impl_literal_composited_validation {
    (
        $CompositedValidateTrait:ident,
        $ValidateTrait:ident,
        $ErrorParams:tt,
        $composited_validation_method:ident,
        $validation_method:ident,
        $type:ty
    ) => {
        impl<T> $CompositedValidateTrait<$type> for T
        where
            T: $ValidateTrait<$type>,
        {
            fn $composited_validation_method(
                &self,
                limit: $type,
            ) -> Result<(), crate::validation::Multiple<$ErrorParams>> {
                self.$validation_method(limit)
                    .map_err(|error| crate::validation::Multiple::Single(error))
            }
        }
    };
}

macro_rules! impl_composited_validation1 {
    ($CompositedValidateTrait:ident, $ValidateTrait:ident, $ErrorParams:tt, $composited_validation_method:ident, $validation_method:ident) => {
        pub trait $CompositedValidateTrait<T> {
            fn $composited_validation_method(
                &self,
                limit: T,
            ) -> Result<(), crate::validation::Multiple<$ErrorParams>>;
        }

        impl<T, U> $CompositedValidateTrait<T> for Vec<U>
        where
            T: Copy,
            U: $CompositedValidateTrait<T>,
        {
            fn $composited_validation_method(
                &self,
                limit: T,
            ) -> Result<(), crate::validation::Multiple<$ErrorParams>> {
                let mut errors = vec![];
                self.iter().for_each(|item| {
                    item.$composited_validation_method(limit)
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

        impl<T, U, const N: usize> $CompositedValidateTrait<T> for [U; N]
        where
            T: Copy,
            U: $CompositedValidateTrait<T>,
        {
            fn $composited_validation_method(
                &self,
                limit: T,
            ) -> Result<(), crate::validation::Multiple<$ErrorParams>> {
                let mut errors = vec![];
                self.iter().for_each(|item| {
                    item.$composited_validation_method(limit)
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

        impl<T, U> $CompositedValidateTrait<T> for Option<U>
        where
            T: Copy,
            U: $CompositedValidateTrait<T>,
        {
            fn $composited_validation_method(
                &self,
                limit: T,
            ) -> Result<(), crate::validation::Multiple<$ErrorParams>> {
                match self {
                    Some(value) => value.$composited_validation_method(limit),
                    None => Ok(()),
                }
            }
        }
    };
}

pub(crate) use impl_composited_validation1;
pub(crate) use impl_literal_composited_validation;
