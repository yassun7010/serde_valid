pub trait ToTomlString {
    /// Convert to toml string.
    ///
    /// ```rust
    /// use serde::Serialize;
    /// use serde_valid::toml::ToTomlString;
    /// use serde_valid::Validate;
    ///
    /// #[derive(Debug, Validate, Serialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 100)]
    ///     val: i32,
    /// }
    /// let s = TestStruct { val: 10 };
    ///
    /// assert!(s.to_toml_string().is_ok());
    /// ```
    fn to_toml_string(&self) -> Result<String, serde_toml::ser::Error>;

    /// Convert to toml pretty string.
    ///
    /// ```rust
    /// use serde::Serialize;
    /// use serde_valid::toml::ToTomlString;
    /// use serde_valid::Validate;
    ///
    /// #[derive(Debug, Validate, Serialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 100)]
    ///     val: i32,
    /// }
    /// let s = TestStruct { val: 10 };
    ///
    /// assert!(s.to_toml_string_pretty().is_ok());
    /// ```
    fn to_toml_string_pretty(&self) -> Result<String, serde_toml::ser::Error>;
}

impl<T> ToTomlString for T
where
    T: serde::Serialize + crate::Validate,
{
    fn to_toml_string(&self) -> Result<String, serde_toml::ser::Error> {
        serde_toml::to_string(self)
    }

    fn to_toml_string_pretty(&self) -> Result<String, serde_toml::ser::Error> {
        serde_toml::to_string_pretty(self)
    }
}

impl ToTomlString for serde_toml::Value {
    fn to_toml_string(&self) -> Result<String, serde_toml::ser::Error> {
        serde_toml::to_string(self)
    }
    fn to_toml_string_pretty(&self) -> Result<String, serde_toml::ser::Error> {
        serde_toml::to_string_pretty(self)
    }
}
