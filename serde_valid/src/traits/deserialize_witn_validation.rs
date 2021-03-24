pub trait DeserializeWithValidation<T>
where
    Self::Error: std::fmt::Debug + std::fmt::Display + std::error::Error,
{
    type Error;
    fn deserialize_with_validation(self) -> Result<T, crate::Error<Self::Error>>;
}

impl<T> DeserializeWithValidation<T> for serde_json::Value
where
    T: serde::de::DeserializeOwned + crate::Validate,
{
    type Error = serde_json::Error;

    fn deserialize_with_validation(self) -> Result<T, crate::Error<Self::Error>> {
        let model: T = serde_json::from_value(self)?;
        model
            .validate()
            .map_err(|e| crate::Error::ValidationError(e))?;
        Ok(model)
    }
}
