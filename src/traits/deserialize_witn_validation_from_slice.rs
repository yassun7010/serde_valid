pub trait DeserializeWithValidationFromSlice
where
    Self::Error: std::error::Error,
{
    type Error;
    fn deserialize_with_validation_from_slice<'a, T>(
        v: &'a [u8],
    ) -> Result<T, crate::Error<Self::Error>>
    where
        T: serde::de::Deserialize<'a> + crate::Validate;
}

impl DeserializeWithValidationFromSlice for serde_json::Value {
    type Error = serde_json::Error;

    fn deserialize_with_validation_from_slice<'a, T>(
        v: &'a [u8],
    ) -> Result<T, crate::Error<Self::Error>>
    where
        T: serde::de::Deserialize<'a> + crate::Validate,
    {
        let model: T = serde_json::from_slice(v)?;
        model
            .validate()
            .map_err(|e| crate::Error::ValidationError(e))?;
        Ok(model)
    }
}
