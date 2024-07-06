pub trait ToYamlString {
    /// Convert to yaml string.
    ///
    /// ```rust
    /// use serde::Serialize;
    /// use serde_valid::yaml::ToYamlString;
    /// use serde_valid::Validate;
    ///
    /// #[derive(Debug, Validate, Serialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 100)]
    ///     val: i32,
    /// }
    /// let s = TestStruct { val: 10 };
    ///
    /// assert!(s.to_yaml_string().is_ok());
    /// ```
    fn to_yaml_string(&self) -> Result<String, serde_yml::Error>;
}

impl<T> ToYamlString for T
where
    T: serde::Serialize + crate::Validate,
{
    fn to_yaml_string(&self) -> Result<String, serde_yml::Error> {
        serde_yml::to_string(self)
    }
}

impl ToYamlString for serde_yml::Value {
    fn to_yaml_string(&self) -> Result<String, serde_yml::Error> {
        serde_yml::to_string(self)
    }
}
