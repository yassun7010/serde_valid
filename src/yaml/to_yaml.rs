pub trait ToYamlString {
    /// Convert to yaml string.
    ///
    /// ```rust
    /// use serde::Serialize;
    /// use serde_valid::yaml::ToYamlString;
    /// use serde_valid::Validate;
    ///
    /// #[derive(Debug, Validate, Serialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 100)]
    ///     val: i32,
    /// }
    /// let s = TestStruct { val: 10 };
    ///
    /// assert!(s.to_yaml_string().is_ok());
    /// ```
    fn to_yaml_string(&self) -> Result<String, serde_yaml::Error>;
}

pub trait ToYamlValue {
    /// Convert to yaml string.
    ///
    /// ```rust
    /// use serde::Serialize;
    /// use serde_valid::yaml::ToYamlValue;
    /// use serde_valid::Validate;
    ///
    /// #[derive(Debug, Validate, Serialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 100)]
    ///     val: i32,
    /// }
    /// let s = TestStruct { val: 10 };
    ///
    /// assert!(s.to_yaml_value().is_ok());
    /// ```
    fn to_yaml_value(&self) -> Result<serde_yaml::Value, serde_yaml::Error>;
}

pub trait ToYamlWriter {
    /// Convert to yaml writer.
    ///
    /// ```should_panic
    /// use std::fs::File;
    /// use serde::Serialize;
    /// use serde_valid::yaml::ToYamlWriter;
    /// use serde_valid::Validate;
    ///
    /// #[derive(Debug, Validate, Serialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 100)]
    ///     val: i32,
    /// }
    /// let s = TestStruct { val: 10 };
    ///
    /// assert!(s.to_yaml_writer(File::open("foo.txt").unwrap()).is_ok());
    /// ```
    fn to_yaml_writer<W>(&self, writer: W) -> Result<(), serde_yaml::Error>
    where
        W: std::io::Write;
}

impl<T> ToYamlString for T
where
    T: serde::Serialize + crate::Validate,
{
    fn to_yaml_string(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }
}

impl<T> ToYamlValue for T
where
    T: serde::Serialize + crate::Validate,
{
    fn to_yaml_value(&self) -> Result<serde_yaml::Value, serde_yaml::Error> {
        serde_yaml::to_value(self)
    }
}

impl<T> ToYamlWriter for T
where
    T: serde::Serialize + crate::Validate,
{
    fn to_yaml_writer<W>(&self, writer: W) -> Result<(), serde_yaml::Error>
    where
        W: std::io::Write,
    {
        serde_yaml::to_writer(writer, self)
    }
}

impl ToYamlString for serde_yaml::Value {
    fn to_yaml_string(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }
}

impl ToYamlWriter for serde_yaml::Value {
    fn to_yaml_writer<W>(&self, writer: W) -> Result<(), serde_yaml::Error>
    where
        W: std::io::Write,
    {
        serde_yaml::to_writer(writer, self)
    }
}
