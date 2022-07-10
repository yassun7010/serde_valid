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

macro_rules! impl_literal_multi_validation {
    (
        $MultiValidateTrait:ident,
        $ValidateTrait:ident,
        $ErrorParams:tt,
        $multi_validation_method:ident,
        $validation_method:ident,
        $type:tt
    ) => {
        impl<T> $MultiValidateTrait<$type> for T
        where
            T: $ValidateTrait<$type>,
        {
            fn $multi_validation_method(
                &self,
                limit: $type,
            ) -> Result<(), crate::validation::Multiple<$ErrorParams>> {
                self.$validation_method(limit)
                    .map_err(|error| crate::validation::Multiple::Single(error))
            }
        }
    };
}

macro_rules! impl_multi_validation1 {
    ($MultiValidateTrait:ident, $ValidateTrait:ident, $ErrorParams:tt, $multi_validation_method:ident, $validation_method:ident) => {
        pub trait $MultiValidateTrait<T> {
            fn $multi_validation_method(
                &self,
                limit: T,
            ) -> Result<(), crate::validation::Multiple<$ErrorParams>>;
        }

        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            i8
        );
        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            i16
        );
        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            i32
        );
        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            i64
        );
        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            i128
        );
        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            isize
        );
        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            u8
        );
        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            u16
        );
        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            u32
        );
        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            u64
        );
        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            u128
        );
        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            usize
        );
        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            NonZeroI8
        );
        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            NonZeroI16
        );
        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            NonZeroI32
        );
        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            NonZeroI64
        );
        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            NonZeroI128
        );
        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            NonZeroIsize
        );
        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            NonZeroU8
        );
        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            NonZeroU16
        );
        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            NonZeroU32
        );
        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            NonZeroU64
        );
        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            NonZeroU128
        );
        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            NonZeroUsize
        );
        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            f32
        );
        impl_literal_multi_validation!(
            $MultiValidateTrait,
            $ValidateTrait,
            $ErrorParams,
            $multi_validation_method,
            $validation_method,
            f64
        );

        impl<T, U> $MultiValidateTrait<T> for Vec<U>
        where
            T: Copy,
            U: $MultiValidateTrait<T>,
        {
            fn $multi_validation_method(
                &self,
                limit: T,
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

        impl<T, U, const N: usize> $MultiValidateTrait<T> for [U; N]
        where
            T: Copy,
            U: $MultiValidateTrait<T>,
        {
            fn $multi_validation_method(
                &self,
                limit: T,
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

        impl<T, U> $MultiValidateTrait<T> for Option<U>
        where
            T: Copy,
            U: $MultiValidateTrait<T>,
        {
            fn $multi_validation_method(
                &self,
                limit: T,
            ) -> Result<(), crate::validation::Multiple<$ErrorParams>> {
                match self {
                    Some(value) => value.$multi_validation_method(limit),
                    None => Ok(()),
                }
            }
        }
    };
}

impl_multi_validation1!(
    ValidateMultiMaximum,
    ValidateMaximum,
    MaximumErrorParams,
    validate_multi_maximum,
    validate_maximum
);

impl_multi_validation1!(
    ValidateMultiMinimum,
    ValidateMinimum,
    MinimumErrorParams,
    validate_multi_minimum,
    validate_minimum
);

impl_multi_validation1!(
    ValidateMultiExclusiveMaximum,
    ValidateExclusiveMaximum,
    ExclusiveMaximumErrorParams,
    validate_multi_exclusive_maximum,
    validate_exclusive_maximum
);

impl_multi_validation1!(
    ValidateMultiExclusiveMinimum,
    ValidateExclusiveMinimum,
    ExclusiveMinimumErrorParams,
    validate_multi_exclusive_minimum,
    validate_exclusive_minimum
);
