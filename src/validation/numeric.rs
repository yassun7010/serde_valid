mod exclusive_maximum;
mod exclusive_minimum;
mod maximum;
mod minimum;
mod multiple_of;
pub use exclusive_maximum::ValidateNumericExclusiveMaximum;
pub use exclusive_minimum::ValidateNumericExclusiveMinimum;
pub use maximum::ValidateNumericMaximum;
pub use minimum::ValidateNumericMinimum;
pub use multiple_of::ValidateNumericMultipleOf;

macro_rules! impl_validate_numeric_range {
    ($tt:tt) => {
        impl<T, U> $tt<T> for Vec<U>
        where
            T: PartialOrd + PartialEq + Copy,
            U: $tt<T>,
        {
            fn check(&self, limit: T) -> bool {
                for item in self {
                    if !item.check(limit) {
                        return false;
                    }
                }

                true
            }
        }

        impl<T, U, const N: usize> $tt<T> for [U; N]
        where
            T: PartialOrd + PartialEq + Copy,
            U: $tt<T>,
        {
            fn check(&self, limit: T) -> bool {
                for item in self {
                    if !item.check(limit) {
                        return false;
                    }
                }

                true
            }
        }

        impl<T, U> $tt<T> for Option<U>
        where
            T: PartialOrd + PartialEq,
            U: $tt<T>,
        {
            fn check(&self, limit: T) -> bool {
                match self {
                    Some(value) => value.check(limit),
                    None => true,
                }
            }
        }
    };
}

impl_validate_numeric_range!(ValidateNumericMaximum);
impl_validate_numeric_range!(ValidateNumericMinimum);
impl_validate_numeric_range!(ValidateNumericExclusiveMaximum);
impl_validate_numeric_range!(ValidateNumericExclusiveMinimum);
