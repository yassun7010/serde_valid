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
    /// struct TestStruct<'a> {
    ///     #[validate(min_length = 1)]
    ///     val: &'a str,
    /// }
    ///
    /// let s = TestStruct::from_json_str(r#"{ "val": "abcde" }"#);
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
        model.validate().map_err(crate::Error::ValidationError)?;
        Ok(model)
    }
}
