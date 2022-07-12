use crate::error::ToDefaultMessage;
use crate::validation::Number;

macro_rules! struct_numeric_range_error_params {
    (
        #[derive(Debug, Clone, Copy)]
        #[default_message=$default_message:literal]
        pub struct $ErrorParams:ident {
            $limit:ident: Number,
        }
    ) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $ErrorParams {
            $limit: Number,
        }

        impl $ErrorParams {
            pub fn new<N: Into<Number>>($limit: N) -> Self {
                Self {
                    $limit: $limit.into(),
                }
            }

            #[allow(dead_code)]
            pub fn $limit(&self) -> Number {
                self.$limit
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
    #[derive(Debug, Clone, Copy)]
    #[default_message = "the number must be `>= {}`."]
    pub struct MinimumErrorParams {
        minimum: Number,
    }
);

struct_numeric_range_error_params!(
    #[derive(Debug, Clone, Copy)]
    #[default_message = "the number must be `<= {}`."]
    pub struct MaximumErrorParams {
        maximum: Number,
    }
);

struct_numeric_range_error_params!(
    #[derive(Debug, Clone, Copy)]
    #[default_message = "the number must be `> {}`."]
    pub struct ExclusiveMinimumErrorParams {
        exclusive_minimum: Number,
    }
);

struct_numeric_range_error_params!(
    #[derive(Debug, Clone, Copy)]
    #[default_message = "the number must be `< {}`."]
    pub struct ExclusiveMaximumErrorParams {
        exclusive_maximum: Number,
    }
);
