use crate::traits::{
    DeserializeWithValidationFromReader, DeserializeWithValidationFromSlice,
    DeserializeWithValidationFromStr, DeserializeWithValidationFromValue,
};

pub trait FromJson
where
    Self: Sized,
{
    /// Convert from json reader.
    ///
    /// ```should_panic
    /// use std::fs::File;
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::json::FromJson;
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 2000)]
    ///     val: i32,
    /// }
    ///
    /// let s = TestStruct::from_json_reader(File::open("foo.txt").unwrap());
    ///
    /// assert!(s.is_ok())
    /// ```
    fn from_json_reader<R>(reader: R) -> Result<Self, crate::Error<serde_json::Error>>
    where
        R: std::io::Read;

    /// Convert from json slice.
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::json::FromJson;
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 2000)]
    ///     val: i32,
    /// }
    ///
    /// let s = TestStruct::from_json_slice(b"{ \"val\": 1234 }");
    ///
    /// assert!(s.is_ok())
    /// ```
    fn from_json_slice(slice: &[u8]) -> Result<Self, crate::Error<serde_json::Error>>;

    /// Convert from json str.
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::json::{json, FromJson};
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 2000)]
    ///     val: i32,
    /// }
    ///
    /// let s = TestStruct::from_json_str(&serde_json::to_string(&json!({ "val": 1234 })).unwrap());
    ///
    /// assert!(s.is_ok())
    /// ```
    fn from_json_str(str: &str) -> Result<Self, crate::Error<serde_json::Error>>;

    /// Convert from [`serde_json::Value`](serde_json::Value).
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::json::{json, FromJson};
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 2000)]
    ///     val: i32,
    /// }
    ///
    /// let s = TestStruct::from_json_value(json!({ "val": 1234 }));
    ///
    /// assert!(s.is_ok())
    /// ```
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
