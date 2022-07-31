use super::message::ToDefaultMessage;
use crate::validation::{Number, Pattern};

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
            #[inline]
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
            #[inline]
            fn to_default_message(&self) -> String {
                format!($default_message)
            }
        }
    };
}

// Number
struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The number must be `>= {}`."]
    pub struct MinimumErrorParams {
        pub minimum: Number,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The number must be `<= {}`."]
    pub struct MaximumErrorParams {
        pub maximum: Number,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The number must be `> {}`."]
    pub struct ExclusiveMinimumErrorParams {
        pub exclusive_minimum: Number,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The number must be `< {}`."]
    pub struct ExclusiveMaximumErrorParams {
        pub exclusive_maximum: Number,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The value must be multiple of `{}`."]
    pub struct MultipleOfErrorParams {
        pub multiple_of: Number,
    }
);

// String
struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The length of the value must be `>= {}`."]
    pub struct MinLengthErrorParams {
        pub min_length: usize,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The length of the value must be `<= {}`."]
    pub struct MaxLengthErrorParams {
        pub max_length: usize,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The value must match the pattern of \"{}\"."]
    pub struct PatternErrorParams {
        pub pattern: Pattern,
    }
);

// Array
struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The length of the items must be `<= {}`."]
    pub struct MaxItemsErrorParams {
        pub max_items: usize,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The length of the items must be `>= {}`."]
    pub struct MinItemsErrorParams {
        pub min_items: usize,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The items must be unique."]
    pub struct UniqueItemsErrorParams {}
);

// Object
struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The size of the properties must be `<= {}`."]
    pub struct MaxPropertiesErrorParams {
        pub max_properties: usize,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The size of the properties must be `>= {}`."]
    pub struct MinPropertiesErrorParams {
        pub min_properties: usize,
    }
);
