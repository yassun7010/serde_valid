pub trait ToJson
where
    Self: Sized,
{
    fn to_json_string(&self) -> Result<String, serde_json::Error>;

    fn to_json_string_pretty(&self) -> Result<String, serde_json::Error>;

    fn to_json_value(&self) -> Result<serde_json::Value, serde_json::Error>;

    fn to_json_writer<W>(&self, writer: W) -> Result<(), serde_json::Error>
    where
        W: std::io::Write;

    fn to_json_writer_pretty<W>(&self, writer: W) -> Result<(), serde_json::Error>
    where
        W: std::io::Write;
}

impl<T> ToJson for T
where
    T: serde::Serialize + crate::Validate,
{
    fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    fn to_json_string_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    fn to_json_value(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }

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
