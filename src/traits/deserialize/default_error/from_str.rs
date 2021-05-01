pub trait DeserializeWithValidationFromStr<T>
where
    Self::Error: std::error::Error,
{
    type Error;

    fn deserialize_with_validation_from_str(str: &str) -> Result<T, crate::Error<Self::Error>>;
}

impl<T> DeserializeWithValidationFromStr<T> for serde_json::Value
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    type Error = serde_json::Error;

    fn deserialize_with_validation_from_str(str: &str) -> Result<T, crate::Error<Self::Error>> {
        let model: T = serde_json::from_str(str)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}

#[cfg(feature = "yaml")]
impl<T> DeserializeWithValidationFromStr<T> for serde_yaml::Value
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    type Error = serde_yaml::Error;

    fn deserialize_with_validation_from_str(str: &str) -> Result<T, crate::Error<Self::Error>> {
        let model: T = serde_yaml::from_str(str)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}

#[cfg(feature = "toml")]
impl<T> DeserializeWithValidationFromStr<T> for serde_toml::Value
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    type Error = serde_toml::de::Error;

    fn deserialize_with_validation_from_str(str: &str) -> Result<T, crate::Error<Self::Error>> {
        let model: T = serde_toml::from_str(str)?;
        model
            .validate()
            .map_err(|err| crate::Error::ValidationError(err))?;
        Ok(model)
    }
}
