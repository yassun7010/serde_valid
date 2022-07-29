pub trait FromJsonReader
where
    Self: Sized,
{
    /// Convert from json reader.
    ///
    /// ```should_panic
    /// use std::fs::File;
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::json::FromJsonReader;
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
}

pub trait FromJsonSlice<'de>
where
    Self: Sized,
{
    /// Convert from json slice.
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::json::FromJsonSlice;
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
    fn from_json_slice(slice: &'de [u8]) -> Result<Self, crate::Error<serde_json::Error>>;
}

pub trait FromJsonStr<'de>
where
    Self: Sized,
{
    /// Convert from json str.
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::json::{json, FromJsonStr};
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
    fn from_json_str(str: &'de str) -> Result<Self, crate::Error<serde_json::Error>>;
}

pub trait FromJsonValue
where
    Self: Sized,
{
    /// Convert from [`serde_json::Value`](serde_json::Value).
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::json::{json, FromJsonValue};
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

impl<T> FromJsonReader for T
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    fn from_json_reader<R>(reader: R) -> Result<Self, crate::Error<serde_json::Error>>
    where
        R: std::io::Read,
    {
        let model: T = serde_json::from_reader(reader)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}

impl<'de, T> FromJsonSlice<'de> for T
where
    T: serde::de::Deserialize<'de> + crate::Validate,
{
    fn from_json_slice(slice: &'de [u8]) -> Result<Self, crate::Error<serde_json::Error>> {
        let model: T = serde_json::from_slice(slice)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}

impl<'de, T> FromJsonStr<'de> for T
where
    T: serde::de::Deserialize<'de> + crate::Validate,
{
    fn from_json_str(str: &'de str) -> Result<Self, crate::Error<serde_json::Error>> {
        let model: Self = serde_json::from_str(str)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}

impl<T> FromJsonValue for T
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    fn from_json_value(value: serde_json::Value) -> Result<Self, crate::Error<serde_json::Error>> {
        let model: T = serde_json::from_value(value)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}
