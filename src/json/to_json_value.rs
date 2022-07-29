pub trait ToJsonValue {
    /// Convert to json string.
    ///
    /// ```rust
    /// use serde::Serialize;
    /// use serde_valid::json::ToJsonValue;
    /// use serde_valid::Validate;
    ///
    /// #[derive(Debug, Validate, Serialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 100)]
    ///     val: i32,
    /// }
    /// let s = TestStruct { val: 10 };
    ///
    /// assert!(s.to_json_value().is_ok());
    /// ```
    fn to_json_value(&self) -> Result<serde_json::Value, serde_json::Error>;
}

impl<T> ToJsonValue for T
where
    T: serde::Serialize + crate::Validate,
{
    fn to_json_value(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}
