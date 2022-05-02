use crate::{
    DeserializeWithValidationFromReader, DeserializeWithValidationFromSlice,
    DeserializeWithValidationFromStr, DeserializeWithValidationFromValue, Error, Validate,
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
