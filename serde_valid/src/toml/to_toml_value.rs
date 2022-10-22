pub trait ToTomlValue {
    /// Convert to toml string.
    ///
    /// ```rust
    /// use serde::Serialize;
    /// use serde_valid::toml::ToTomlValue;
    /// use serde_valid::Validate;
    ///
    /// #[derive(Debug, Validate, Serialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 100)]
    ///     val: i32,
    /// }
    /// let s = TestStruct { val: 10 };
    ///
    /// assert!(s.to_toml_value().is_ok());
    /// ```
    fn to_toml_value(&self) -> Result<serde_toml::Value, serde_toml::ser::Error>;
}

impl<T> ToTomlValue for T
where
    T: serde::Serialize + crate::Validate,
{
    fn to_toml_value(&self) -> Result<serde_toml::Value, serde_toml::ser::Error> {
        serde_toml::Value::try_from(self)
    }
}
