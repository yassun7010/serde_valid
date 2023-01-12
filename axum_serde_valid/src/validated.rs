#[cfg(not(any(feature = "jsonschema", feature = "aide")))]
mod deserialize {
    pub trait Deserialize: serde::de::DeserializeOwned + serde_valid::Validate {}

    impl<T> Deserialize for T where T: serde::de::DeserializeOwned + serde_valid::Validate {}
}

#[cfg(any(feature = "jsonschema", feature = "aide"))]
mod deserialize {
    pub trait Deserialize:
        serde::de::DeserializeOwned + serde_valid::Validate + schemars::JsonSchema
    {
    }

    impl<T> Deserialize for T where
        T: serde::de::DeserializeOwned + serde_valid::Validate + schemars::JsonSchema
    {
    }
}

pub use deserialize::Deserialize;
