mod generic;
mod message;

use crate::validation::{self, Number, Pattern};
pub use generic::EnumerateErrorParams;
pub use message::{Message, ToDefaultMessage};

#[derive(Debug, thiserror::Error)]
pub enum Error<E>
where
    E: 'static + std::error::Error,
{
    #[error(transparent)]
    DeserializeError(#[from] E),

    #[error(transparent)]
    ValidationError(validation::Errors),
}

impl<E> Error<E>
where
    E: 'static + std::error::Error,
{
    pub fn is_serde_error(&self) -> bool {
        match self {
            Self::DeserializeError(_) => true,
            Self::ValidationError(_) => false,
        }
    }

    pub fn as_serde_error(&self) -> Option<&E> {
        match self {
            Self::DeserializeError(error) => Some(error),
            Self::ValidationError(_) => None,
        }
    }

    pub fn is_validation_errors(&self) -> bool {
        match self {
            Self::DeserializeError(_) => false,
            Self::ValidationError(_) => true,
        }
    }

    pub fn as_validation_errors(&self) -> Option<&validation::Errors> {
        match self {
            Self::DeserializeError(_) => None,
            Self::ValidationError(error) => Some(error),
        }
    }
}

macro_rules! struct_error_params {
    (
        #[derive(Debug, Clone)]
        #[default_message=$default_message:literal]
        pub struct $ErrorParams:ident {
            pub $limit:ident: $type:ty,
        }
    ) => {
        #[derive(Debug, Clone)]
        pub struct $ErrorParams {
            pub $limit: $type,
        }

        impl $ErrorParams {
            pub fn new<N: Into<$type>>($limit: N) -> Self {
                Self {
                    $limit: $limit.into(),
                }
            }
        }

        impl ToDefaultMessage for $ErrorParams {
            fn to_default_message(&self) -> String {
                format!($default_message, self.$limit)
            }
        }
    };

    (
        #[derive(Debug, Clone)]
        #[default_message=$default_message:literal]
        pub struct $ErrorParams:ident {
        }
    ) => {
        #[derive(Debug, Clone)]
        pub struct $ErrorParams {}

        impl ToDefaultMessage for $ErrorParams {
            fn to_default_message(&self) -> String {
                format!($default_message)
            }
        }
    };
}

// Number
struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "the number must be `>= {}`."]
    pub struct MinimumErrorParams {
        pub minimum: Number,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "the number must be `<= {}`."]
    pub struct MaximumErrorParams {
        pub maximum: Number,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "the number must be `> {}`."]
    pub struct ExclusiveMinimumErrorParams {
        pub exclusive_minimum: Number,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "the number must be `< {}`."]
    pub struct ExclusiveMaximumErrorParams {
        pub exclusive_maximum: Number,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "the value must be multiple of `{}`."]
    pub struct MultipleOfErrorParams {
        pub multiple_of: Number,
    }
);

// String
struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "the length of the value must be `>= {}`."]
    pub struct MinLengthErrorParams {
        pub min_length: usize,
    }
);
struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "the length of the value must be `<= {}`."]
    pub struct MaxLengthErrorParams {
        pub max_length: usize,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "the value must match the pattern of \"{}\"."]
    pub struct PatternErrorParams {
        pub pattern: Pattern,
    }
);

// Array
struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "the length of the items must be `<= {}`."]
    pub struct MaxItemsErrorParams {
        pub max_items: usize,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "the length of the items must be `>= {}`."]
    pub struct MinItemsErrorParams {
        pub min_items: usize,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "items must be unique."]
    pub struct UniqueItemsErrorParams {}
);

// Object
struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "the size of the properties must be `<= {}`."]
    pub struct MaxPropertiesErrorParams {
        pub max_properties: usize,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "the size of the properties must be `>= {}`."]
    pub struct MinPropertiesErrorParams {
        pub min_properties: usize,
    }
);
