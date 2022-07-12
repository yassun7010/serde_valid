use crate::error::ToDefaultMessage;
use crate::validation::Number;

macro_rules! struct_numeric_range_error_params {
    (
        #[derive(Debug, Clone)]
        #[default_message=$default_message:literal]
        pub struct $ErrorParams:ident {
            pub $limit:ident: Number,
        }
    ) => {
        #[derive(Debug, Clone)]
        pub struct $ErrorParams {
            pub $limit: Number,
        }

        impl $ErrorParams {
            pub fn new<N: Into<Number>>($limit: N) -> Self {
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
}

struct_numeric_range_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "the number must be `>= {}`."]
    pub struct MinimumErrorParams {
        pub minimum: Number,
    }
);

struct_numeric_range_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "the number must be `<= {}`."]
    pub struct MaximumErrorParams {
        pub maximum: Number,
    }
);

struct_numeric_range_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "the number must be `> {}`."]
    pub struct ExclusiveMinimumErrorParams {
        pub exclusive_minimum: Number,
    }
);

struct_numeric_range_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "the number must be `< {}`."]
    pub struct ExclusiveMaximumErrorParams {
        pub exclusive_maximum: Number,
    }
);
