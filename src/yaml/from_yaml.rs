use crate::{
    DeserializeWithValidationFromReader, DeserializeWithValidationFromSlice,
    DeserializeWithValidationFromStr, DeserializeWithValidationFromValue,
};

pub trait FromYaml
where
    Self: Sized,
{
    /// Convert from yaml reader.
    ///
    /// ```should_panic
    /// use std::fs::File;
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::yaml::FromYaml;
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 2000)]
    ///     val: i32,
    /// }
    ///
    /// let s = TestStruct::from_yaml_reader(File::open("foo.txt").unwrap());
    ///
    /// assert!(s.is_ok())
    /// ```
    fn from_yaml_reader<R>(reader: R) -> Result<Self, crate::Error<serde_yaml::Error>>
    where
        R: std::io::Read;

    /// Convert from yaml slice.
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::yaml::FromYaml;
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 10)]
    ///     val: i32,
    /// }
    ///
    /// let s = TestStruct::from_yaml_slice(b"---\nval: 10\n");
    ///
    /// assert!(s.is_ok())
    /// ```
    fn from_yaml_slice(slice: &[u8]) -> Result<Self, crate::Error<serde_yaml::Error>>;

    /// Convert from yaml str.
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::yaml::{FromYaml};
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 2000)]
    ///     val: i32,
    /// }
    ///
    /// let s = TestStruct::from_yaml_str("---\nval: 10\n");
    ///
    /// assert!(s.is_ok())
    /// ```
    fn from_yaml_str(str: &str) -> Result<Self, crate::Error<serde_yaml::Error>>;

    /// Convert from [`serde_yaml::Value`](serde_yaml::Value).
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::yaml::{FromYaml, Value};
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 2000)]
    ///     val: i32,
    /// }
    ///
    /// let s = TestStruct::from_yaml_value(serde_yaml::from_str("val: 5").unwrap());
    ///
    /// assert!(s.is_ok())
    /// ```
    fn from_yaml_value(value: serde_yaml::Value) -> Result<Self, crate::Error<serde_yaml::Error>>;
}

impl<T> FromYaml for T
where
    T: crate::Validate
        + DeserializeWithValidationFromReader<serde_yaml::Value, serde_yaml::Error>
        + DeserializeWithValidationFromSlice<serde_yaml::Value, serde_yaml::Error>
        + DeserializeWithValidationFromStr<serde_yaml::Value, serde_yaml::Error>
        + DeserializeWithValidationFromValue<serde_yaml::Value, serde_yaml::Error>,
{
    fn from_yaml_reader<R>(reader: R) -> Result<Self, crate::Error<serde_yaml::Error>>
    where
        R: std::io::Read,
    {
        T::deserialize_with_validation_from_reader(reader)
    }

    fn from_yaml_slice(slice: &[u8]) -> Result<Self, crate::Error<serde_yaml::Error>> {
        T::deserialize_with_validation_from_slice(slice)
    }

    fn from_yaml_str(str: &str) -> Result<Self, crate::Error<serde_yaml::Error>> {
        T::deserialize_with_validation_from_str(str)
    }

    fn from_yaml_value(value: serde_yaml::Value) -> Result<Self, crate::Error<serde_yaml::Error>> {
        T::deserialize_with_validation_from_value(value)
    }
}
