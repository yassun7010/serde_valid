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
            .map_err(|e| crate::Error::ValidationError(e))?;
        Ok(model)
    }
}
