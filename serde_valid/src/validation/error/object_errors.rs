use serde::ser::SerializeStruct;

use super::{PropertyErrorsMap, VecErrors};

#[derive(Debug, Clone, thiserror::Error)]
pub struct ObjectErrors<Err = crate::validation::Error> {
    pub errors: VecErrors<Err>,
    pub properties: PropertyErrorsMap<Err>,
}

impl<Err> serde::Serialize for ObjectErrors<Err>
where
    Err: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut object_errors = serializer.serialize_struct("ObjectErrors", 2)?;
        object_errors.serialize_field("errors", &self.errors)?;
        object_errors.serialize_field("properties", &self.properties)?;
        object_errors.end()
    }
}

impl<Err> ObjectErrors<Err> {
    pub fn new(errors: VecErrors<Err>, properties: PropertyErrorsMap<Err>) -> Self {
        Self { errors, properties }
    }
}

impl<Err> std::fmt::Display for ObjectErrors<Err>
where
    Err: std::fmt::Display + serde::Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(&self) {
            Ok(json_string) => {
                write!(f, "{}", json_string)
            }
            Err(_) => Err(std::fmt::Error),
        }
    }
}
