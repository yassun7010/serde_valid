pub trait DeserializeWithValidationFromReader<V, E>
where
    Self: Sized,
    E: std::error::Error,
{
    fn deserialize_with_validation_from_reader<R>(reader: R) -> Result<Self, crate::Error<E>>
    where
        R: std::io::Read;
}

impl<T> DeserializeWithValidationFromReader<serde_json::Value, serde_json::Error> for T
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    fn deserialize_with_validation_from_reader<R>(
        reader: R,
    ) -> Result<T, crate::Error<serde_json::Error>>
    where
        R: std::io::Read,
    {
        let model: T = serde_json::from_reader(reader)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}

#[cfg(feature = "yaml")]
impl<T> DeserializeWithValidationFromReader<T> for serde_yaml::Value
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    type Error = serde_yaml::Error;

    fn deserialize_with_validation_from_reader<R>(rdr: R) -> Result<T, crate::Error<Self::Error>>
    where
        R: std::io::Read,
    {
        let model: T = serde_yaml::from_reader(rdr)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}

#[cfg(feature = "toml")]
impl<T> DeserializeWithValidationFromReader<T> for serde_toml::Value
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    type Error = serde_toml::de::Error;

    fn deserialize_with_validation_from_reader<R>(rdr: R) -> Result<T, crate::Error<Self::Error>>
    where
        R: std::io::Read,
    {
        use serde::de::Error;

        let mut buffer = String::new();
        let mut reader = rdr;
        reader
            .read_to_string(&mut buffer)
            .map_err(|err| serde_toml::de::Error::custom(err))?;

        let model: T = serde_toml::from_str(&buffer)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}
