pub trait ToJsonString {
    /// Convert to json string.
    ///
    /// ```rust
    /// use serde::Serialize;
    /// use serde_valid::json::ToJsonString;
    /// use serde_valid::Validate;
    ///
    /// #[derive(Debug, Validate, Serialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 100)]
    ///     val: i32,
    /// }
    /// let s = TestStruct { val: 10 };
    ///
    /// assert!(s.to_json_string().is_ok());
    /// ```
    fn to_json_string(&self) -> Result<String, serde_json::Error>;

    /// Convert to json pretty string.
    ///
    /// ```rust
    /// use serde::Serialize;
    /// use serde_valid::json::ToJsonString;
    /// use serde_valid::Validate;
    ///
    /// #[derive(Debug, Validate, Serialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 100)]
    ///     val: i32,
    /// }
    /// let s = TestStruct { val: 10 };
    ///
    /// assert!(s.to_json_string_pretty().is_ok());
    /// ```
    fn to_json_string_pretty(&self) -> Result<String, serde_json::Error>;
}

pub trait ToJsonValue {
    /// Convert to json string.
    ///
    /// ```rust
    /// use serde::Serialize;
    /// use serde_valid::json::ToJsonValue;
    /// use serde_valid::Validate;
    ///
    /// #[derive(Debug, Validate, Serialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 100)]
    ///     val: i32,
    /// }
    /// let s = TestStruct { val: 10 };
    ///
    /// assert!(s.to_json_value().is_ok());
    /// ```
    fn to_json_value(&self) -> Result<serde_json::Value, serde_json::Error>;
}

pub trait ToJsonWriter {
    /// Convert to json writer.
    ///
    /// ```should_panic
    /// use std::fs::File;
    /// use serde::Serialize;
    /// use serde_valid::json::ToJsonWriter;
    /// use serde_valid::Validate;
    ///
    /// #[derive(Debug, Validate, Serialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 100)]
    ///     val: i32,
    /// }
    /// let s = TestStruct { val: 10 };
    ///
    /// assert!(s.to_json_writer(File::open("foo.txt").unwrap()).is_ok());
    /// ```
    fn to_json_writer<W>(&self, writer: W) -> Result<(), serde_json::Error>
    where
        W: std::io::Write;

    /// Convert to pretty json writer.
    ///
    /// ```should_panic
    /// use std::fs::File;
    /// use serde::Serialize;
    /// use serde_valid::json::ToJsonWriter;
    /// use serde_valid::Validate;
    ///
    /// #[derive(Debug, Validate, Serialize)]
    /// struct TestStruct {
    ///     #[validate(maximum = 100)]
    ///     val: i32,
    /// }
    /// let s = TestStruct { val: 10 };
    ///
    /// assert!(s.to_json_writer_pretty(File::open("foo.txt").unwrap()).is_ok());
    /// ```
    fn to_json_writer_pretty<W>(&self, writer: W) -> Result<(), serde_json::Error>
    where
        W: std::io::Write;
}

impl<T> ToJsonString for T
where
    T: serde::Serialize + crate::Validate,
{
    fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    fn to_json_string_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

impl<T> ToJsonValue for T
where
    T: serde::Serialize + crate::Validate,
{
    fn to_json_value(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}

impl<T> ToJsonWriter for T
where
    T: serde::Serialize + crate::Validate,
{
    fn to_json_writer<W>(&self, writer: W) -> Result<(), serde_json::Error>
    where
        W: std::io::Write,
    {
        serde_json::to_writer(writer, self)
    }

    fn to_json_writer_pretty<W>(&self, writer: W) -> Result<(), serde_json::Error>
    where
        W: std::io::Write,
    {
        serde_json::to_writer_pretty(writer, self)
    }
}

impl ToJsonString for serde_json::Value {
    fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    fn to_json_string_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

impl ToJsonWriter for serde_json::Value {
    fn to_json_writer<W>(&self, writer: W) -> Result<(), serde_json::Error>
    where
        W: std::io::Write,
    {
        serde_json::to_writer(writer, self)
    }

    fn to_json_writer_pretty<W>(&self, writer: W) -> Result<(), serde_json::Error>
    where
        W: std::io::Write,
    {
        serde_json::to_writer_pretty(writer, self)
    }
}
