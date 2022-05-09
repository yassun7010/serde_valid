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
impl<T> DeserializeWithValidationFromSlice<serde_yaml::Value, serde_yaml::Error> for T
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    fn deserialize_with_validation_from_slice(
        slice: &[u8],
    ) -> Result<T, crate::Error<serde_yaml::Error>> {
        let model: T = serde_yaml::from_slice(slice)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}

#[cfg(feature = "toml")]
impl<T> DeserializeWithValidationFromSlice<serde_toml::Value, serde_toml::de::Error> for T
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    fn deserialize_with_validation_from_slice(
        slice: &[u8],
    ) -> Result<T, crate::Error<serde_toml::de::Error>>
    where
        T: serde::de::DeserializeOwned + crate::Validate,
    {
        let model: T = serde_toml::from_slice(slice)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}
