mod max_items;
mod min_items;
mod unique_items;

pub use max_items::ValidateMaxItems;
pub use min_items::ValidateMinItems;
pub use unique_items::ValidateUniqueItems;

use crate::{MaxItemsErrorParams, MinItemsErrorParams};

macro_rules! impl_validate_array_length_items {
    ($ValidateTrait:tt, $ErrorParams:tt, $validation_method:ident) => {
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

impl_validate_array_length_items!(ValidateMaxItems, MaxItemsErrorParams, validate_max_items);
impl_validate_array_length_items!(ValidateMinItems, MinItemsErrorParams, validate_min_items);
