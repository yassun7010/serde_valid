mod error;
mod traits;
pub mod validation;
pub use error::Error;
pub use traits::*;
pub use validation::{
    validate_array_items, validate_array_unique_items, validate_generic_enumerate,
    validate_numeric_multiple_of, validate_numeric_range, validate_object_properties,
    validate_string_length, validate_string_pattern, FieldName, Limit,
};

pub fn from_value<T, V>(value: V) -> Result<T, self::Error<V::Error>>
where
    T: serde::de::DeserializeOwned,
    V: DeserializeWithValidationFromValue<T>,
    V::Error: std::error::Error,
{
    value.deserialize_with_validation_from_value()
}

pub fn from_str<T, V>(str: &str) -> Result<T, self::Error<V::Error>>
where
    T: serde::de::DeserializeOwned,
    V: DeserializeWithValidationFromStr<T>,
    V::Error: std::error::Error,
{
    V::deserialize_with_validation_from_str(str)
}

pub fn from_slice<'a, T, V>(v: &'a [u8]) -> Result<T, self::Error<V::Error>>
where
    T: serde::de::DeserializeOwned + Validate,
    V: DeserializeWithValidationFromSlice<T>,
    V::Error: std::error::Error,
{
    V::deserialize_with_validation_from_slice(v)
}

pub fn from_reader<R, T, V>(rdr: R) -> Result<T, self::Error<V::Error>>
where
    R: std::io::Read,
    T: serde::de::DeserializeOwned,
    V: DeserializeWithValidationFromReader<T>,
    V::Error: std::error::Error,
{
    V::deserialize_with_validation_from_reader(rdr)
}

pub trait Validate {
    fn validate(&self) -> Result<(), self::validation::Errors>;
}

pub use serde_valid_derive::Validate;

pub mod json {
    pub use serde_json::{
        json, Deserializer, Error, Map, Number, Serializer, StreamDeserializer, Value,
    };
}

#[cfg(feature = "yaml")]
pub mod yaml {
    pub use serde_yaml::{mapping, Error, Index, Location, Mapping, Number, Sequence, Value};
}

#[cfg(feature = "toml")]
pub mod toml {
    pub use serde_toml::{map, toml, value, Deserializer, Serializer, Value};
    mod ser {
        pub use serde_toml::ser::{Error, Serializer};
    }
    mod de {
        pub use serde_toml::de::{Deserializer, Error};
    }
}
