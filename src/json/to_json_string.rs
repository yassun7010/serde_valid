pub trait ToJsonString {
    /// Convert to json string.
    ///
    /// ```rust
    /// use serde::Serialize;
    /// use serde_valid::json::ToJsonString;
    /// use serde_valid::Validate;
    ///
    /// #[derive(Debug, Validate, Serialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 100)]
    ///     val: i32,
    /// }
    /// let s = TestStruct { val: 10 };
    ///
    /// assert!(s.to_json_string().is_ok());
    /// ```
    fn to_json_string(&self) -> Result<String, serde_json::Error>;

    /// Convert to json pretty string.
    ///
    /// ```rust
    /// use serde::Serialize;
    /// use serde_valid::json::ToJsonString;
    /// use serde_valid::Validate;
    ///
    /// #[derive(Debug, Validate, Serialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 100)]
    ///     val: i32,
    /// }
    /// let s = TestStruct { val: 10 };
    ///
    /// assert!(s.to_json_string_pretty().is_ok());
    /// ```
    fn to_json_string_pretty(&self) -> Result<String, serde_json::Error>;
}

impl<T> ToJsonString for T
where
    T: serde::Serialize + crate::Validate,
{
    fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    fn to_json_string_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

impl ToJsonString for serde_json::Value {
    fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    fn to_json_string_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
