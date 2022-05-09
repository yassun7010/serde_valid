use crate::{
    DeserializeWithValidationFromReader, DeserializeWithValidationFromSlice,
    DeserializeWithValidationFromStr, DeserializeWithValidationFromValue,
};

pub trait FromToml
where
    Self: Sized,
{
    /// Convert from toml reader.
    ///
    /// ```should_panic
    /// use std::fs::File;
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::toml::FromToml;
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 2000)]
    ///     val: i32,
    /// }
    ///
    /// let s = TestStruct::from_toml_reader(File::open("foo.txt").unwrap());
    ///
    /// assert!(s.is_ok())
    /// ```
    fn from_toml_reader<R>(reader: R) -> Result<Self, crate::Error<serde_toml::de::Error>>
    where
        R: std::io::Read;

    /// Convert from toml slice.
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::toml::FromToml;
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 10)]
    ///     val: i32,
    /// }
    ///
    /// let s = TestStruct::from_toml_slice(b"val= 10\n");
    ///
    /// assert!(s.is_ok())
    /// ```
    fn from_toml_slice(slice: &[u8]) -> Result<Self, crate::Error<serde_toml::de::Error>>;

    /// Convert from toml str.
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::toml::FromToml;
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 2000)]
    ///     val: i32,
    /// }
    ///
    /// let s = TestStruct::from_toml_str("val = 10\n");
    ///
    /// assert!(s.is_ok())
    /// ```
    fn from_toml_str(str: &str) -> Result<Self, crate::Error<serde_toml::de::Error>>;

    /// Convert from [`serde_toml::Value`](serde_toml::Value).
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::toml::{FromToml, Value};
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 2000)]
    ///     val: i32,
    /// }
    ///
    /// let s = TestStruct::from_toml_value(serde_toml::from_str("val = 5").unwrap());
    ///
    /// assert!(s.is_ok())
    /// ```
    fn from_toml_value(
        value: serde_toml::Value,
    ) -> Result<Self, crate::Error<serde_toml::de::Error>>;
}

impl<T> FromToml for T
where
    T: crate::Validate
        + DeserializeWithValidationFromReader<serde_toml::Value, serde_toml::de::Error>
        + DeserializeWithValidationFromSlice<serde_toml::Value, serde_toml::de::Error>
        + DeserializeWithValidationFromStr<serde_toml::Value, serde_toml::de::Error>
        + DeserializeWithValidationFromValue<serde_toml::Value, serde_toml::de::Error>,
{
    fn from_toml_reader<R>(reader: R) -> Result<Self, crate::Error<serde_toml::de::Error>>
    where
        R: std::io::Read,
    {
        T::deserialize_with_validation_from_reader(reader)
    }

    fn from_toml_slice(slice: &[u8]) -> Result<Self, crate::Error<serde_toml::de::Error>> {
        T::deserialize_with_validation_from_slice(slice)
    }

    fn from_toml_str(str: &str) -> Result<Self, crate::Error<serde_toml::de::Error>> {
        T::deserialize_with_validation_from_str(str)
    }

    fn from_toml_value(
        value: serde_toml::Value,
    ) -> Result<Self, crate::Error<serde_toml::de::Error>> {
        T::deserialize_with_validation_from_value(value)
    }
}
