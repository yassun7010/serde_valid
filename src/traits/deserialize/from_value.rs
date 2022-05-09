pub trait DeserializeWithValidationFromValue<V, E>
where
    Self: Sized,
    E: std::error::Error,
{
    fn deserialize_with_validation_from_value(value: V) -> Result<Self, crate::Error<E>>;
}

impl<T> DeserializeWithValidationFromValue<serde_json::Value, serde_json::Error> for T
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    fn deserialize_with_validation_from_value(
        value: serde_json::Value,
    ) -> Result<T, crate::Error<serde_json::Error>> {
        let model: T = serde_json::from_value(value)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}

#[cfg(feature = "yaml")]
impl<T> DeserializeWithValidationFromValue<serde_yaml::Value, serde_yaml::Error> for T
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    fn deserialize_with_validation_from_value(
        value: serde_yaml::Value,
    ) -> Result<T, crate::Error<serde_yaml::Error>> {
        let model: T = serde_yaml::from_value(value)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}

#[cfg(feature = "toml")]
impl<T> DeserializeWithValidationFromValue<serde_toml::Value, serde_toml::de::Error> for T
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    fn deserialize_with_validation_from_value(
        value: serde_toml::Value,
    ) -> Result<T, crate::Error<serde_toml::de::Error>> {
        let model: T = serde::Deserialize::deserialize(value)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}
