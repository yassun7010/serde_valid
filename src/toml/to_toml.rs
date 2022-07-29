use serde::ser::Error;

pub trait ToTomlString {
    /// Convert to toml string.
    ///
    /// ```rust
    /// use serde::Serialize;
    /// use serde_valid::toml::ToTomlString;
    /// use serde_valid::Validate;
    ///
    /// #[derive(Debug, Validate, Serialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 100)]
    ///     val: i32,
    /// }
    /// let s = TestStruct { val: 10 };
    ///
    /// assert!(s.to_toml_string().is_ok());
    /// ```
    fn to_toml_string(&self) -> Result<String, serde_toml::ser::Error>;

    /// Convert to toml pretty string.
    ///
    /// ```rust
    /// use serde::Serialize;
    /// use serde_valid::toml::ToTomlString;
    /// use serde_valid::Validate;
    ///
    /// #[derive(Debug, Validate, Serialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 100)]
    ///     val: i32,
    /// }
    /// let s = TestStruct { val: 10 };
    ///
    /// assert!(s.to_toml_string_pretty().is_ok());
    /// ```
    fn to_toml_string_pretty(&self) -> Result<String, serde_toml::ser::Error>;
}

pub trait ToTomlValue {
    /// Convert to toml string.
    ///
    /// ```rust
    /// use serde::Serialize;
    /// use serde_valid::toml::ToTomlValue;
    /// use serde_valid::Validate;
    ///
    /// #[derive(Debug, Validate, Serialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 100)]
    ///     val: i32,
    /// }
    /// let s = TestStruct { val: 10 };
    ///
    /// assert!(s.to_toml_value().is_ok());
    /// ```
    fn to_toml_value(&self) -> Result<serde_toml::Value, serde_toml::ser::Error>;
}

pub trait ToTomlWriter {
    /// Convert to toml writer.
    ///
    /// ```should_panic
    /// use std::fs::File;
    /// use serde::Serialize;
    /// use serde_valid::toml::ToTomlWriter;
    /// use serde_valid::Validate;
    ///
    /// #[derive(Debug, Validate, Serialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 100)]
    ///     val: i32,
    /// }
    /// let s = TestStruct { val: 10 };
    ///
    /// assert!(s.to_toml_writer(File::open("foo.txt").unwrap()).is_ok());
    /// ```
    fn to_toml_writer<W>(&self, writer: W) -> Result<(), serde_toml::ser::Error>
    where
        W: std::io::Write;

    /// Convert to pretty toml writer.
    ///
    /// ```should_panic
    /// use std::fs::File;
    /// use serde::Serialize;
    /// use serde_valid::toml::ToTomlWriter;
    /// use serde_valid::Validate;
    ///
    /// #[derive(Debug, Validate, Serialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 100)]
    ///     val: i32,
    /// }
    /// let s = TestStruct { val: 10 };
    ///
    /// assert!(s.to_toml_writer_pretty(File::open("foo.txt").unwrap()).is_ok());
    /// ```
    fn to_toml_writer_pretty<W>(&self, writer: W) -> Result<(), serde_toml::ser::Error>
    where
        W: std::io::Write;
}

impl<T> ToTomlString for T
where
    T: serde::Serialize + crate::Validate,
{
    fn to_toml_string(&self) -> Result<String, serde_toml::ser::Error> {
        serde_toml::to_string(self)
    }

    fn to_toml_string_pretty(&self) -> Result<String, serde_toml::ser::Error> {
        serde_toml::to_string_pretty(self)
    }
}

impl<T> ToTomlValue for T
where
    T: serde::Serialize + crate::Validate,
{
    fn to_toml_value(&self) -> Result<serde_toml::Value, serde_toml::ser::Error> {
        serde_toml::Value::try_from(self)
    }
}

impl<T> ToTomlWriter for T
where
    T: serde::Serialize + crate::Validate,
{
    fn to_toml_writer<W>(&self, writer: W) -> Result<(), serde_toml::ser::Error>
    where
        W: std::io::Write,
    {
        let mut writer = writer;
        match writer.write(&self.to_toml_string()?.into_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(serde_toml::ser::Error::custom(err.to_string())),
        }
    }

    fn to_toml_writer_pretty<W>(&self, writer: W) -> Result<(), serde_toml::ser::Error>
    where
        W: std::io::Write,
    {
        let mut writer = writer;
        match writer.write(&self.to_toml_string_pretty()?.into_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(serde_toml::ser::Error::custom(err.to_string())),
        }
    }
}

impl ToTomlString for serde_toml::Value {
    fn to_toml_string(&self) -> Result<String, serde_toml::ser::Error> {
        serde_toml::to_string(self)
    }
    fn to_toml_string_pretty(&self) -> Result<String, serde_toml::ser::Error> {
        serde_toml::to_string_pretty(self)
    }
}

impl ToTomlWriter for serde_toml::Value {
    fn to_toml_writer<W>(&self, writer: W) -> Result<(), serde_toml::ser::Error>
    where
        W: std::io::Write,
    {
        let mut writer = writer;
        match writer.write(&self.to_toml_string()?.into_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(serde_toml::ser::Error::custom(err.to_string())),
        }
    }

    fn to_toml_writer_pretty<W>(&self, writer: W) -> Result<(), serde_toml::ser::Error>
    where
        W: std::io::Write,
    {
        let mut writer = writer;
        match writer.write(&self.to_toml_string_pretty()?.into_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(serde_toml::ser::Error::custom(err.to_string())),
        }
    }
}
