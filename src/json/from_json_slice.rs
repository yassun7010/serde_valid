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
