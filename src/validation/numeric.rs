mod exclusive_maximum;
mod exclusive_minimum;
mod maximum;
mod minimum;
mod multiple_of;
pub use exclusive_maximum::ValidateExclusiveMaximum;
pub use exclusive_minimum::ValidateExclusiveMinimum;
pub use maximum::ValidateMaximum;
pub use minimum::ValidateMinimum;
pub use multiple_of::ValidateMultipleOf;
use std::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

use crate::{
    ExclusiveMaximumErrorParams, ExclusiveMinimumErrorParams, MaximumErrorParams,
    MinimumErrorParams,
};

macro_rules! impl_literal_composited_validation {
    (
        $CompositedValidateTrait:ident,
        $ValidateTrait:ident,
        $ErrorParams:tt,
        $composited_validation_method:ident,
        $validation_method:ident,
        $type:tt
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

        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            i8
        );
        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            i16
        );
        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            i32
        );
        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            i64
        );
        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            i128
        );
        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            isize
        );
        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            u8
        );
        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            u16
        );
        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            u32
        );
        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            u64
        );
        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            u128
        );
        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            usize
        );
        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            NonZeroI8
        );
        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            NonZeroI16
        );
        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            NonZeroI32
        );
        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            NonZeroI64
        );
        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            NonZeroI128
        );
        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            NonZeroIsize
        );
        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            NonZeroU8
        );
        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            NonZeroU16
        );
        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            NonZeroU32
        );
        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            NonZeroU64
        );
        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            NonZeroU128
        );
        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            NonZeroUsize
        );
        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            f32
        );
        impl_literal_composited_validation!(
            $CompositedValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $composited_validation_method,
            $validation_method,
            f64
        );

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

impl_composited_validation1!(
    ValidateCompositedMaximum,
    ValidateMaximum,
    MaximumErrorParams,
    validate_composited_maximum,
    validate_maximum
);

impl_composited_validation1!(
    ValidateCompositedMinimum,
    ValidateMinimum,
    MinimumErrorParams,
    validate_composited_minimum,
    validate_minimum
);

impl_composited_validation1!(
    ValidateCompositedExclusiveMaximum,
    ValidateExclusiveMaximum,
    ExclusiveMaximumErrorParams,
    validate_composited_exclusive_maximum,
    validate_exclusive_maximum
);

impl_composited_validation1!(
    ValidateCompositedExclusiveMinimum,
    ValidateExclusiveMinimum,
    ExclusiveMinimumErrorParams,
    validate_composited_exclusive_minimum,
    validate_exclusive_minimum
);
