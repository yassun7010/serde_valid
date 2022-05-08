mod deserialize;
pub mod error;
mod traits;
pub mod validation;

pub use deserialize::*;
pub use error::{
    EnumerateErrorParams, Error, ExclusiveMaximumErrorParams, ExclusiveMinimumErrorParams,
    MaxItemsErrorParams, MaxLengthErrorParams, MaxPropertiesErrorParams, MaximumErrorParams,
    MinItemsErrorParams, MinLengthErrorParams, MinPropertiesErrorParams, MinimumErrorParams,
    MultipleOfErrorParams, PatternErrorParams, UniqueItemsErrorParams,
};
pub use traits::*;
pub use validation::{
    ValidateArrayMaxItems, ValidateArrayMinItems, ValidateArrayUniqueItems,
    ValidateGenericEnumerate, ValidateNumericExclusiveMaximum, ValidateNumericExclusiveMinimum,
    ValidateNumericMaximum, ValidateNumericMinimum, ValidateNumericMultipleOf,
    ValidateObjectMaxProperties, ValidateObjectMinProperties, ValidateStringMaxLength,
    ValidateStringMinLength, ValidateStringPattern,
};

pub trait Validate {
    fn validate(&self) -> std::result::Result<(), self::validation::Errors>;
}

pub use serde_valid_derive::Validate;
