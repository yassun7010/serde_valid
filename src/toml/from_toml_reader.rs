pub trait FromTomlReader
where
    Self: Sized,
{
    /// Convert from toml reader.
    ///
    /// ```should_panic
    /// use std::fs::File;
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::toml::FromTomlReader;
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 2000)]
    ///     val: i32,
    /// }
    ///
    /// let s = TestStruct::from_toml_reader(File::open("foo.txt").unwrap());
    ///
    /// assert!(s.is_ok())
    /// ```
    fn from_toml_reader<R>(reader: R) -> Result<Self, crate::Error<serde_toml::de::Error>>
    where
        R: std::io::Read;
}

impl<T> FromTomlReader for T
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    fn from_toml_reader<R>(reader: R) -> Result<Self, crate::Error<serde_toml::de::Error>>
    where
        R: std::io::Read,
    {
        use serde::de::Error;

        let mut buffer = String::new();
        let mut reader = reader;
        reader
            .read_to_string(&mut buffer)
            .map_err(|err| serde_toml::de::Error::custom(err))?;

        let model: T = serde_toml::from_str(&buffer)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}
