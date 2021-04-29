pub trait DeserializeWithValidationFromReader<T>
where
    Self::Error: std::error::Error,
{
    type Error;
    fn deserialize_with_validation_from_reader<R>(str: R) -> Result<T, Self::Error>
    where
        R: std::io::Read;
}

impl<T> DeserializeWithValidationFromReader<T> for serde_json::Value
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    type Error = serde_json::Error;

    fn deserialize_with_validation_from_reader<R>(rdr: R) -> Result<T, Self::Error>
    where
        R: std::io::Read,
    {
        let model: T = serde_json::from_reader(rdr)?;
        model
            .validate()
            .map_err(|err| serde::de::Error::custom(err))?;
        Ok(model)
    }
}

#[cfg(feature = "yaml")]
impl<T> DeserializeWithValidationFromReader<T> for serde_yaml::Value
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    type Error = serde_yaml::Error;

    fn deserialize_with_validation_from_reader<R>(rdr: R) -> Result<T, Self::Error>
    where
        R: std::io::Read,
    {
        let model: T = serde_yaml::from_reader(rdr)?;
        model
            .validate()
            .map_err(|err| serde::de::Error::custom(err))?;
        Ok(model)
    }
}
