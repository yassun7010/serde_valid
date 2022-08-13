use super::message::ToDefaultMessage;
use crate::validation::{Number, Pattern};

macro_rules! struct_error_params {
    (
        #[derive(Debug, Clone)]
        #[default_message=$default_message:literal]
        pub struct $ErrorParams:ident {
            $limit:ident: $type:ty,
        }
    ) => {
        #[derive(Debug, Clone)]
        pub struct $ErrorParams {
            $limit: $type,
        }

        impl $ErrorParams {
            pub fn new<N: Into<$type>>($limit: N) -> Self {
                Self {
                    $limit: $limit.into(),
                }
            }

            pub fn $limit(&self) -> &$type {
                &self.$limit
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
        #[derive(Debug, Copy, Clone)]
        #[default_message=$default_message:literal]
        pub struct $ErrorParams:ident {
            $limit:ident: $type:ty,
        }
    ) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $ErrorParams {
            $limit: $type,
        }

        impl $ErrorParams {
            pub fn new<N: Into<$type>>($limit: N) -> Self {
                Self {
                    $limit: $limit.into(),
                }
            }

            pub fn $limit(&self) -> $type {
                self.$limit
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
        #[derive(Debug, Copy, Clone)]
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
    #[derive(Debug, Copy, Clone)]
    #[default_message = "The number must be `>= {}`."]
    pub struct MinimumErrorParams {
        minimum: Number,
    }
);

struct_error_params!(
    #[derive(Debug, Copy, Clone)]
    #[default_message = "The number must be `<= {}`."]
    pub struct MaximumErrorParams {
        maximum: Number,
    }
);

struct_error_params!(
    #[derive(Debug, Copy, Clone)]
    #[default_message = "The number must be `> {}`."]
    pub struct ExclusiveMinimumErrorParams {
        exclusive_minimum: Number,
    }
);

struct_error_params!(
    #[derive(Debug, Copy, Clone)]
    #[default_message = "The number must be `< {}`."]
    pub struct ExclusiveMaximumErrorParams {
        exclusive_maximum: Number,
    }
);

struct_error_params!(
    #[derive(Debug, Copy, Clone)]
    #[default_message = "The value must be multiple of `{}`."]
    pub struct MultipleOfErrorParams {
        multiple_of: Number,
    }
);

// String
struct_error_params!(
    #[derive(Debug, Copy, Clone)]
    #[default_message = "The length of the value must be `>= {}`."]
    pub struct MinLengthErrorParams {
        min_length: usize,
    }
);

struct_error_params!(
    #[derive(Debug, Copy, Clone)]
    #[default_message = "The length of the value must be `<= {}`."]
    pub struct MaxLengthErrorParams {
        max_length: usize,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The value must match the pattern of \"{}\"."]
    pub struct PatternErrorParams {
        pattern: Pattern,
    }
);

// Array
struct_error_params!(
    #[derive(Debug, Copy, Clone)]
    #[default_message = "The length of the items must be `<= {}`."]
    pub struct MaxItemsErrorParams {
        max_items: usize,
    }
);

struct_error_params!(
    #[derive(Debug, Copy, Clone)]
    #[default_message = "The length of the items must be `>= {}`."]
    pub struct MinItemsErrorParams {
        min_items: usize,
    }
);

struct_error_params!(
    #[derive(Debug, Copy, Clone)]
    #[default_message = "The items must be unique."]
    pub struct UniqueItemsErrorParams {}
);

// Object
struct_error_params!(
    #[derive(Debug, Copy, Clone)]
    #[default_message = "The size of the properties must be `<= {}`."]
    pub struct MaxPropertiesErrorParams {
        max_properties: usize,
    }
);

struct_error_params!(
    #[derive(Debug, Copy, Clone)]
    #[default_message = "The size of the properties must be `>= {}`."]
    pub struct MinPropertiesErrorParams {
        min_properties: usize,
    }
);
