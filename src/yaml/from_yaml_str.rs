pub trait FromYamlStr<'de>
where
    Self: Sized,
{
    /// Convert from yaml str.
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::yaml::FromYamlStr;
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct<'a> {
    ///     #[validate(min_length = 1)]
    ///     val: &'a str,
    /// }
    ///
    /// let s = TestStruct::from_yaml_str("---\nval: abcde\n");
    ///
    /// assert!(s.is_ok())
    /// ```
    fn from_yaml_str(str: &'de str) -> Result<Self, crate::Error<serde_yaml::Error>>;
}

impl<'de, T> FromYamlStr<'de> for T
where
    T: serde::de::Deserialize<'de> + crate::Validate,
{
    fn from_yaml_str(str: &'de str) -> Result<Self, crate::Error<serde_yaml::Error>> {
        let model: T = serde_yaml::from_str(str)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}
