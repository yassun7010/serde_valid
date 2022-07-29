pub trait FromTomlSlice<'de>
where
    Self: Sized,
{
    /// Convert from toml slice.
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::toml::FromTomlSlice;
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct<'a> {
    ///     #[validate(min_length = 1)]
    ///     val: &'a str,
    /// }
    ///
    /// let s = TestStruct::from_toml_slice(br#"val= "abcde""#);
    ///
    /// assert!(s.is_ok())
    /// ```
    fn from_toml_slice(slice: &'de [u8]) -> Result<Self, crate::Error<serde_toml::de::Error>>;
}

impl<'de, T> FromTomlSlice<'de> for T
where
    T: serde::de::Deserialize<'de> + crate::Validate,
{
    fn from_toml_slice(slice: &'de [u8]) -> Result<Self, crate::Error<serde_toml::de::Error>> {
        let model: T = serde_toml::from_slice(slice)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}
