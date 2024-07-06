pub trait FromYamlReader
where
    Self: Sized,
{
    /// Convert from yaml reader.
    ///
    /// ```should_panic
    /// use std::fs::File;
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::yaml::FromYamlReader;
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 2000)]
    ///     val: i32,
    /// }
    ///
    /// let s = TestStruct::from_yaml_reader(File::open("foo.txt").unwrap());
    ///
    /// assert!(s.is_ok())
    /// ```
    fn from_yaml_reader<R>(reader: R) -> Result<Self, crate::Error<serde_yml::Error>>
    where
        R: std::io::Read;
}

impl<T> FromYamlReader for T
where
    for<'de> T: serde::de::Deserialize<'de>,
    T: crate::Validate,
{
    fn from_yaml_reader<R>(reader: R) -> Result<Self, crate::Error<serde_yml::Error>>
    where
        R: std::io::Read,
    {
        let model: T = serde_yml::from_reader(reader)?;
        model.validate().map_err(crate::Error::ValidationError)?;
        Ok(model)
    }
}
