pub trait FromTomlStr<'de>
where
    Self: Sized,
{
    /// Convert from toml str.
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::toml::FromTomlStr;
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
    fn from_toml_str(str: &'de str) -> Result<Self, crate::Error<serde_toml::de::Error>>;
}

impl<'de, T> FromTomlStr<'de> for T
where
    T: serde::de::Deserialize<'de> + crate::Validate,
{
    fn from_toml_str(str: &'de str) -> Result<Self, crate::Error<serde_toml::de::Error>> {
        let model: T = serde_toml::from_str(str)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}
