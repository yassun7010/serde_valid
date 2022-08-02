pub trait FromJsonValue
where
    Self: Sized,
{
    /// Convert from [`serde_json::Value`](serde_json::Value).
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::json::{json, FromJsonValue};
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 2000)]
    ///     val: i32,
    /// }
    ///
    /// let s = TestStruct::from_json_value(json!({ "val": 1234 }));
    ///
    /// assert!(s.is_ok())
    /// ```
    fn from_json_value(value: serde_json::Value) -> Result<Self, crate::Error<serde_json::Error>>;
}

impl<T> FromJsonValue for T
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    fn from_json_value(value: serde_json::Value) -> Result<Self, crate::Error<serde_json::Error>> {
        let model: T = serde_json::from_value(value)?;
        model
            .validate()
            .map_err(crate::Error::ValidationError)?;
        Ok(model)
    }
}
