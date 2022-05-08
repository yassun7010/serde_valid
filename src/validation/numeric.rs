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

use crate::{
    ExclusiveMaximumErrorParams, ExclusiveMinimumErrorParams, MaximumErrorParams,
    MinimumErrorParams,
};

macro_rules! impl_validate_numeric_range {
    ($ValidateTrait:tt, $ErrorParams:tt, $validation_method:ident) => {
        impl<T, U> $ValidateTrait<T> for Vec<U>
        where
            T: PartialOrd + PartialEq + Copy,
            U: $ValidateTrait<T>,
        {
            fn $validation_method(&self, limit: T) -> Result<(), $ErrorParams> {
                for item in self {
                    item.$validation_method(limit)?
                }

                Ok(())
            }
        }

        impl<T, U, const N: usize> $ValidateTrait<T> for [U; N]
        where
            T: PartialOrd + PartialEq + Copy,
            U: $ValidateTrait<T>,
        {
            fn $validation_method(&self, limit: T) -> Result<(), $ErrorParams> {
                for item in self {
                    item.$validation_method(limit)?
                }

                Ok(())
            }
        }

        impl<T, U> $ValidateTrait<T> for Option<U>
        where
            T: PartialOrd + PartialEq,
            U: $ValidateTrait<T>,
        {
            fn $validation_method(&self, limit: T) -> Result<(), $ErrorParams> {
                match self {
                    Some(value) => value.$validation_method(limit),
                    None => Ok(()),
                }
            }
        }
    };
}

impl_validate_numeric_range!(ValidateMaximum, MaximumErrorParams, validate_maximum);
impl_validate_numeric_range!(ValidateMinimum, MinimumErrorParams, validate_minimum);
impl_validate_numeric_range!(
    ValidateExclusiveMaximum,
    ExclusiveMaximumErrorParams,
    validate_exclusive_maximum
);
impl_validate_numeric_range!(
    ValidateExclusiveMinimum,
    ExclusiveMinimumErrorParams,
    validate_exclusive_minimum
);
