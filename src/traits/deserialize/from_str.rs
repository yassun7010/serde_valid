pub trait DeserializeWithValidationFromStr<V, E>
where
    Self: Sized,
    E: std::error::Error,
{
    fn deserialize_with_validation_from_str(str: &str) -> Result<Self, crate::Error<E>>;
}

impl<T> DeserializeWithValidationFromStr<serde_json::Value, serde_json::Error> for T
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    fn deserialize_with_validation_from_str(
        str: &str,
    ) -> Result<Self, crate::Error<serde_json::Error>> {
        let model: Self = serde_json::from_str(str)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}

#[cfg(feature = "yaml")]
impl<T> DeserializeWithValidationFromStr<serde_yaml::Value, serde_yaml::Error> for T
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    fn deserialize_with_validation_from_str(
        str: &str,
    ) -> Result<T, crate::Error<serde_yaml::Error>> {
        let model: T = serde_yaml::from_str(str)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}

#[cfg(feature = "toml")]
impl<T> DeserializeWithValidationFromStr<serde_toml::Value, serde_toml::de::Error> for T
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    fn deserialize_with_validation_from_str(
        str: &str,
    ) -> Result<T, crate::Error<serde_toml::de::Error>> {
        let model: T = serde_toml::from_str(str)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}
