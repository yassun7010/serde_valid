use serde::ser::Error;

use super::ToTomlString;

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
