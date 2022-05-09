use crate::{
    DeserializeWithValidationFromReader, DeserializeWithValidationFromSlice,
    DeserializeWithValidationFromStr, DeserializeWithValidationFromValue,
};

pub trait FromJson
where
    Self: Sized,
{
    fn from_json_reader<R>(reader: R) -> Result<Self, crate::Error<serde_json::Error>>
    where
        R: std::io::Read;

    fn from_json_slice(slice: &[u8]) -> Result<Self, crate::Error<serde_json::Error>>;

    fn from_json_str(str: &str) -> Result<Self, crate::Error<serde_json::Error>>;

    fn from_json_value(value: serde_json::Value) -> Result<Self, crate::Error<serde_json::Error>>;
}

impl<T> FromJson for T
where
    T: crate::Validate
        + DeserializeWithValidationFromReader<serde_json::Value, serde_json::Error>
        + DeserializeWithValidationFromSlice<serde_json::Value, serde_json::Error>
        + DeserializeWithValidationFromStr<serde_json::Value, serde_json::Error>
        + DeserializeWithValidationFromValue<serde_json::Value, serde_json::Error>,
{
    fn from_json_reader<R>(reader: R) -> Result<Self, crate::Error<serde_json::Error>>
    where
        R: std::io::Read,
    {
        T::deserialize_with_validation_from_reader(reader)
    }

    fn from_json_slice(slice: &[u8]) -> Result<Self, crate::Error<serde_json::Error>> {
        T::deserialize_with_validation_from_slice(slice)
    }

    fn from_json_str(str: &str) -> Result<Self, crate::Error<serde_json::Error>> {
        T::deserialize_with_validation_from_str(str)
    }

    fn from_json_value(value: serde_json::Value) -> Result<Self, crate::Error<serde_json::Error>> {
        T::deserialize_with_validation_from_value(value)
    }
}
