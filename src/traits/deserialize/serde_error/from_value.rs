pub trait DeserializeWithValidationFromValue<T>
where
    Self::Error: std::error::Error,
{
    type Error;

    fn deserialize_with_validation_from_value(self) -> Result<T, Self::Error>;
}

impl<T> DeserializeWithValidationFromValue<T> for serde_json::Value
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    type Error = serde_json::Error;

    fn deserialize_with_validation_from_value(self) -> Result<T, Self::Error> {
        let model: T = serde_json::from_value(self)?;
        model
            .validate()
            .map_err(|err| serde::de::Error::custom(err))?;
        Ok(model)
    }
}

#[cfg(feature = "yaml")]
impl<T> DeserializeWithValidationFromValue<T> for serde_yaml::Value
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    type Error = serde_yaml::Error;

    fn deserialize_with_validation_from_value(self) -> Result<T, Self::Error> {
        let model: T = serde_yaml::from_value(self)?;
        model
            .validate()
            .map_err(|err| serde::de::Error::custom(err))?;
        Ok(model)
    }
}
