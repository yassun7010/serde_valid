use serde::ser::Error;

pub trait ToToml {
    fn to_toml_string(&self) -> Result<String, serde_toml::ser::Error>;

    fn to_toml_string_pretty(&self) -> Result<String, serde_toml::ser::Error>;

    fn to_toml_value(&self) -> Result<serde_toml::Value, serde_toml::ser::Error>;

    fn to_toml_writer<W>(&self, writer: W) -> Result<(), serde_toml::ser::Error>
    where
        W: std::io::Write;

    fn to_toml_writer_pretty<W>(&self, writer: W) -> Result<(), serde_toml::ser::Error>
    where
        W: std::io::Write;
}

impl<T> ToToml for T
where
    T: serde::Serialize + crate::Validate,
{
    fn to_toml_string(&self) -> Result<String, serde_toml::ser::Error> {
        serde_toml::to_string(self)
    }

    fn to_toml_string_pretty(&self) -> Result<String, serde_toml::ser::Error> {
        serde_toml::to_string_pretty(self)
    }

    fn to_toml_value(&self) -> Result<serde_toml::Value, serde_toml::ser::Error> {
        serde_toml::Value::try_from(self)
    }

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
