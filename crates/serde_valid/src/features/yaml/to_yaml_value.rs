pub trait ToYamlValue {
    /// Convert to yaml string.
    ///
    /// ```rust
    /// use serde::Serialize;
    /// use serde_valid::yaml::ToYamlValue;
    /// use serde_valid::Validate;
    ///
    /// #[derive(Debug, Validate, Serialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 100)]
    ///     val: i32,
    /// }
    /// let s = TestStruct { val: 10 };
    ///
    /// assert!(s.to_yaml_value().is_ok());
    /// ```
    fn to_yaml_value(&self) -> Result<serde_yaml::Value, serde_yaml::Error>;
}

impl<T> ToYamlValue for T
where
    T: serde::Serialize + crate::Validate,
{
    fn to_yaml_value(&self) -> Result<serde_yaml::Value, serde_yaml::Error> {
        serde_yaml::to_value(self)
    }
}
