pub trait FromYaml
where
    Self: Sized,
{
    /// Convert from yaml reader.
    ///
    /// ```should_panic
    /// use std::fs::File;
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::yaml::FromYaml;
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
    fn from_yaml_reader<R>(reader: R) -> Result<Self, crate::Error<serde_yaml::Error>>
    where
        R: std::io::Read;

    /// Convert from yaml slice.
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::yaml::FromYaml;
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 10)]
    ///     val: i32,
    /// }
    ///
    /// let s = TestStruct::from_yaml_slice(b"---\nval: 10\n");
    ///
    /// assert!(s.is_ok())
    /// ```
    fn from_yaml_slice(slice: &[u8]) -> Result<Self, crate::Error<serde_yaml::Error>>;

    /// Convert from yaml str.
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::yaml::FromYaml;
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 2000)]
    ///     val: i32,
    /// }
    ///
    /// let s = TestStruct::from_yaml_str("---\nval: 10\n");
    ///
    /// assert!(s.is_ok())
    /// ```
    fn from_yaml_str(str: &str) -> Result<Self, crate::Error<serde_yaml::Error>>;

    /// Convert from [`serde_yaml::Value`](serde_yaml::Value).
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::yaml::{FromYaml, Value};
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 2000)]
    ///     val: i32,
    /// }
    ///
    /// let s = TestStruct::from_yaml_value(serde_yaml::from_str("val: 5").unwrap());
    ///
    /// assert!(s.is_ok())
    /// ```
    fn from_yaml_value(value: serde_yaml::Value) -> Result<Self, crate::Error<serde_yaml::Error>>;
}

impl<T> FromYaml for T
where
    for<'de> T: serde::de::Deserialize<'de>,
    T: crate::Validate,
{
    fn from_yaml_reader<R>(reader: R) -> Result<Self, crate::Error<serde_yaml::Error>>
    where
        R: std::io::Read,
    {
        let model: T = serde_yaml::from_reader(reader)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }

    fn from_yaml_slice(slice: &[u8]) -> Result<Self, crate::Error<serde_yaml::Error>> {
        let model: T = serde_yaml::from_slice(slice)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }

    fn from_yaml_str(str: &str) -> Result<Self, crate::Error<serde_yaml::Error>> {
        let model: T = serde_yaml::from_str(str)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }

    fn from_yaml_value(value: serde_yaml::Value) -> Result<Self, crate::Error<serde_yaml::Error>> {
        let model: T = serde_yaml::from_value(value)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}
