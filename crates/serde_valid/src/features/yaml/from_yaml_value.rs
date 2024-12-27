pub trait FromYamlValue
where
    Self: Sized,
{
    /// Convert from [`serde_yaml::Value`](serde_yaml::Value).
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::yaml::{FromYamlValue, Value};
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

impl<T> FromYamlValue for T
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    fn from_yaml_value(value: serde_yaml::Value) -> Result<Self, crate::Error<serde_yaml::Error>> {
        let model: T = serde_yaml::from_value(value)?;
        model.validate().map_err(crate::Error::ValidationError)?;
        Ok(model)
    }
}
