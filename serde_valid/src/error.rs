#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Range Error")]
    RangeError,
    #[error("MultipleOf Error")]
    MultipleOfError,
    #[error("Length Error")]
    LengthError,
    #[error("Pattern Error")]
    PatternError,
    #[error("Items Error")]
    ItemsError,
    #[error("UniqueItems Error")]
    UniqueItemsError,
    #[error("Properties Error")]
    PropertiesError,
}
