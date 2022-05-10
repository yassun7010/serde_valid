pub trait ToYaml {
    fn to_yaml_string(&self) -> Result<String, serde_yaml::Error>;

    fn to_yaml_value(&self) -> Result<serde_yaml::Value, serde_yaml::Error>;

    fn to_yaml_writer<W>(&self, writer: W) -> Result<(), serde_yaml::Error>
    where
        W: std::io::Write;
}

impl<T> ToYaml for T
where
    T: serde::Serialize + crate::Validate,
{
    fn to_yaml_string(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }

    fn to_yaml_value(&self) -> Result<serde_yaml::Value, serde_yaml::Error> {
        serde_yaml::to_value(self)
    }

    fn to_yaml_writer<W>(&self, writer: W) -> Result<(), serde_yaml::Error>
    where
        W: std::io::Write,
    {
        serde_yaml::to_writer(writer, self)
    }
}
