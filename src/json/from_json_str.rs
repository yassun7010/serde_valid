pub trait FromJsonStr<'de>
where
    Self: Sized,
{
    /// Convert from json str.
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::json::{json, FromJsonStr};
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 2000)]
    ///     val: i32,
    /// }
    ///
    /// let s = TestStruct::from_json_str(&serde_json::to_string(&json!({ "val": 1234 })).unwrap());
    ///
    /// assert!(s.is_ok())
    /// ```
    fn from_json_str(str: &'de str) -> Result<Self, crate::Error<serde_json::Error>>;
}

impl<'de, T> FromJsonStr<'de> for T
where
    T: serde::de::Deserialize<'de> + crate::Validate,
{
    fn from_json_str(str: &'de str) -> Result<Self, crate::Error<serde_json::Error>> {
        let model: Self = serde_json::from_str(str)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}
