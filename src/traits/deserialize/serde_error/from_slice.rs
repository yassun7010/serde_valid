pub trait DeserializeWithValidationFromSlice<T>
where
    T: serde::de::DeserializeOwned + crate::Validate,
    Self::Error: std::error::Error,
{
    type Error;

    fn deserialize_with_validation_from_slice(v: &[u8]) -> Result<T, Self::Error>;
}

impl<T> DeserializeWithValidationFromSlice<T> for serde_json::Value
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    type Error = serde_json::Error;

    fn deserialize_with_validation_from_slice(v: &[u8]) -> Result<T, Self::Error> {
        let model: T = serde_json::from_slice(v)?;
        model
            .validate()
            .map_err(|err| serde::de::Error::custom(err))?;
        Ok(model)
    }
}

#[cfg(feature = "yaml")]
impl<T> DeserializeWithValidationFromSlice<T> for serde_yaml::Value
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    type Error = serde_yaml::Error;

    fn deserialize_with_validation_from_slice(v: &[u8]) -> Result<T, Self::Error> {
        let model: T = serde_yaml::from_slice(v)?;
        model
            .validate()
            .map_err(|err| serde::de::Error::custom(err))?;
        Ok(model)
    }
}

#[cfg(feature = "toml")]
impl<T> DeserializeWithValidationFromSlice<T> for serde_toml::Value
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    type Error = serde_toml::de::Error;

    fn deserialize_with_validation_from_slice(v: &[u8]) -> Result<T, Self::Error>
    where
        T: serde::de::DeserializeOwned + crate::Validate,
    {
        let model: T = serde_toml::from_slice(v)?;
        model
            .validate()
            .map_err(|err| serde::de::Error::custom(err))?;
        Ok(model)
    }
}
