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
    /// struct TestStruct<'a> {
    ///     #[validate(min_length = 1)]
    ///     val: &'a str,
    /// }
    ///
    /// let s = TestStruct::from_json_slice(br#"{ "val": "abcde" }"#);
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
            .map_err(crate::Error::ValidationError)?;
        Ok(model)
    }
}
