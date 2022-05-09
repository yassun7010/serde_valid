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
pub use serde::{de, ser, Deserialize, Deserializer, Serialize, Serializer};
pub use traits::*;
pub use validation::{
    ValidateEnumerate, ValidateExclusiveMaximum, ValidateExclusiveMinimum, ValidateMaxItems,
    ValidateMaxLength, ValidateMaxProperties, ValidateMaximum, ValidateMinItems, ValidateMinLength,
    ValidateMinProperties, ValidateMinimum, ValidateMultipleOf, ValidatePattern,
    ValidateUniqueItems,
};

pub trait Validate {
    fn validate(&self) -> std::result::Result<(), self::validation::Errors>;
}

pub use serde_valid_derive::Validate;
