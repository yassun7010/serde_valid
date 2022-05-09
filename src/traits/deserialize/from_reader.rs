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
impl<T> DeserializeWithValidationFromReader<serde_yaml::Value, serde_yaml::Error> for T
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    fn deserialize_with_validation_from_reader<R>(
        reader: R,
    ) -> Result<T, crate::Error<serde_yaml::Error>>
    where
        R: std::io::Read,
    {
        let model: T = serde_yaml::from_reader(reader)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}

#[cfg(feature = "toml")]
impl<T> DeserializeWithValidationFromReader<serde_toml::Value, serde_toml::de::Error> for T
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    fn deserialize_with_validation_from_reader<R>(
        reader: R,
    ) -> Result<T, crate::Error<serde_toml::de::Error>>
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
}
