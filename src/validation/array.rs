mod max_items;
mod min_items;
mod unique_items;

pub use max_items::ValidateMaxItems;
pub use min_items::ValidateMinItems;
pub use unique_items::ValidateUniqueItems;

use crate::{MaxItemsErrorParams, MinItemsErrorParams};

macro_rules! impl_validate_array_length_items {
    ($ErrorType:ident) => {
        paste::paste! {
            impl<T> [<Validate $ErrorType>] for Option<T>
            where
                T: [<Validate $ErrorType>],
            {
                fn [<validate_ $ErrorType:snake>] (&self, limit: usize) -> Result<(), [<$ErrorType ErrorParams>]> {
                    match self {
                        Some(value) => value.[<validate_ $ErrorType:snake>](limit),
                        None => Ok(()),
                    }
                }
            }
        }
    };
}

impl_validate_array_length_items!(MaxItems);
impl_validate_array_length_items!(MinItems);
