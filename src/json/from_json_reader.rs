pub trait FromJsonReader
where
    Self: Sized,
{
    /// Convert from json reader.
    ///
    /// ```should_panic
    /// use std::fs::File;
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::json::FromJsonReader;
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 2000)]
    ///     val: i32,
    /// }
    ///
    /// let s = TestStruct::from_json_reader(File::open("foo.txt").unwrap());
    ///
    /// assert!(s.is_ok())
    /// ```
    fn from_json_reader<R>(reader: R) -> Result<Self, crate::Error<serde_json::Error>>
    where
        R: std::io::Read;
}

impl<T> FromJsonReader for T
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    fn from_json_reader<R>(reader: R) -> Result<Self, crate::Error<serde_json::Error>>
    where
        R: std::io::Read,
    {
        let model: T = serde_json::from_reader(reader)?;
        model
            .validate()
            .map_err(crate::Error::ValidationError)?;
        Ok(model)
    }
}
