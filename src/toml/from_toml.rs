pub trait FromToml
where
    Self: Sized,
{
    /// Convert from toml reader.
    ///
    /// ```should_panic
    /// use std::fs::File;
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::toml::FromToml;
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

    /// Convert from toml slice.
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::toml::FromToml;
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 10)]
    ///     val: i32,
    /// }
    ///
    /// let s = TestStruct::from_toml_slice(b"val= 10\n");
    ///
    /// assert!(s.is_ok())
    /// ```
    fn from_toml_slice(slice: &[u8]) -> Result<Self, crate::Error<serde_toml::de::Error>>;

    /// Convert from toml str.
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::toml::FromToml;
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 2000)]
    ///     val: i32,
    /// }
    ///
    /// let s = TestStruct::from_toml_str("val = 10\n");
    ///
    /// assert!(s.is_ok())
    /// ```
    fn from_toml_str(str: &str) -> Result<Self, crate::Error<serde_toml::de::Error>>;

    /// Convert from [`serde_toml::Value`](serde_toml::Value).
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_valid::Validate;
    /// use serde_valid::toml::{FromToml, Value};
    ///
    /// #[derive(Debug, Validate, Deserialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 2000)]
    ///     val: i32,
    /// }
    ///
    /// let s = TestStruct::from_toml_value(serde_toml::from_str("val = 5").unwrap());
    ///
    /// assert!(s.is_ok())
    /// ```
    fn from_toml_value(
        value: serde_toml::Value,
    ) -> Result<Self, crate::Error<serde_toml::de::Error>>;
}

impl<T> FromToml for T
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

    fn from_toml_slice(slice: &[u8]) -> Result<Self, crate::Error<serde_toml::de::Error>> {
        let model: T = serde_toml::from_slice(slice)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }

    fn from_toml_str(str: &str) -> Result<Self, crate::Error<serde_toml::de::Error>> {
        let model: T = serde_toml::from_str(str)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }

    fn from_toml_value(
        value: serde_toml::Value,
    ) -> Result<Self, crate::Error<serde_toml::de::Error>> {
        let model: T = serde::Deserialize::deserialize(value)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}
