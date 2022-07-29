pub trait FromTomlValue
where
    Self: Sized,
{
    /// Convert from [`serde_toml::Value`](serde_toml::Value).
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::toml::{FromTomlValue, Value};
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

impl<T> FromTomlValue for T
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    fn from_toml_value(
        value: serde_toml::Value,
    ) -> Result<Self, crate::Error<serde_toml::de::Error>> {
        let model: T = serde::Deserialize::deserialize(value)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}
