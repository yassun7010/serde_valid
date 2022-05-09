pub trait DeserializeWithValidationFromSlice<V, E>
where
    Self: Sized,
    E: std::error::Error,
{
    fn deserialize_with_validation_from_slice(v: &[u8]) -> Result<Self, crate::Error<E>>;
}

impl<T> DeserializeWithValidationFromSlice<serde_json::Value, serde_json::Error> for T
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    fn deserialize_with_validation_from_slice(
        v: &[u8],
    ) -> Result<T, crate::Error<serde_json::Error>> {
        let model: T = serde_json::from_slice(v)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}

#[cfg(feature = "yaml")]
impl<T> DeserializeWithValidationFromSlice<T> for serde_yaml::Value
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    type Error = serde_yaml::Error;

    fn deserialize_with_validation_from_slice(v: &[u8]) -> Result<T, crate::Error<Self::Error>> {
        let model: T = serde_yaml::from_slice(v)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}

#[cfg(feature = "toml")]
impl<T> DeserializeWithValidationFromSlice<T> for serde_toml::Value
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    type Error = serde_toml::de::Error;

    fn deserialize_with_validation_from_slice(v: &[u8]) -> Result<T, crate::Error<Self::Error>>
    where
        T: serde::de::DeserializeOwned + crate::Validate,
    {
        let model: T = serde_toml::from_slice(v)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}
