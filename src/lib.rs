mod deserialize;
#[cfg(not(serde_error))]
mod error;
mod traits;
pub mod validation;
pub use deserialize::*;
#[cfg(not(serde_error))]
pub use error::Error;
pub use traits::*;
pub use validation::{
    validate_array_items, validate_array_unique_items, validate_generic_enumerate,
    validate_numeric_multiple_of, validate_numeric_range, validate_object_properties,
    validate_string_length, validate_string_pattern, Limit,
};

pub trait Validate {
    fn validate(&self) -> Result<(), self::validation::Errors>;
}

pub use serde_valid_derive::Validate;

pub mod json {
    pub use serde_json::{
        json, Deserializer, Error, Map, Number, Serializer, StreamDeserializer, Value,
    };
}

#[cfg(not(feature = "serde_error"))]
pub type JsonError = Error<serde_json::Value>;

#[cfg(feature = "yaml")]
pub mod yaml {
    pub use serde_yaml::{mapping, Error, Index, Location, Mapping, Number, Sequence, Value};
}

#[cfg(all(feature = "yaml", not(feature = "serde_error")))]
pub type YamlError = Error<serde_yaml::Value>;

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

#[cfg(all(feature = "toml", not(feature = "serde_error")))]
pub type TomlError = Error<serde_toml::Value>;
