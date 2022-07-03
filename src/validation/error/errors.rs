use super::{ArrayErrors, ObjectErrors, VecErrors};

#[derive(Debug, serde::Serialize, thiserror::Error)]
#[serde(untagged)]
pub enum Errors {
    Array(ArrayErrors),
    Object(ObjectErrors),
    #[serde(serialize_with = "serialize")]
    NewType(VecErrors),
}

pub fn serialize<T>(errors: &VecErrors, serializer: T) -> Result<T::Ok, T::Error>
where
    T: serde::ser::Serializer,
{
    serializer.collect_map([("errors", errors)])
}

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Array(errors) => std::fmt::Display::fmt(errors, f),
            Self::Object(errors) => std::fmt::Display::fmt(errors, f),
            Self::NewType(vec_errors) => {
                match serde_json::to_string(
                    &vec_errors
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>(),
                ) {
                    Ok(json_string) => write!(f, "{}", json_string),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
