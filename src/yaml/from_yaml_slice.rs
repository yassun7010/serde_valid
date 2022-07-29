pub trait FromYamlSlice<'de>
where
    Self: Sized,
{
    /// Convert from yaml slice.
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::yaml::FromYamlSlice;
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
    fn from_yaml_slice(slice: &'de [u8]) -> Result<Self, crate::Error<serde_yaml::Error>>;
}

impl<'de, T> FromYamlSlice<'de> for T
where
    T: serde::de::Deserialize<'de> + crate::Validate,
{
    fn from_yaml_slice(slice: &'de [u8]) -> Result<Self, crate::Error<serde_yaml::Error>> {
        let model: T = serde_yaml::from_slice(slice)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}
