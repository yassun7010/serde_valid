use crate::error::ToDefaultMessage;

macro_rules! struct_numeric_range_error_params {
    (
        #[derive(Debug, Clone)]
        #[default_message=$default_message:literal]
        pub struct $ErrorParams:ident {
            $limit:ident: String,
        }
    ) => {
        #[derive(Debug, Clone)]
        pub struct $ErrorParams {
            $limit: String,
        }

        impl $ErrorParams {
            pub fn new<T>($limit: T) -> Self
            where
                T: PartialOrd + PartialEq + ToString,
            {
                Self {
                    $limit: $limit.to_string(),
                }
            }

            #[allow(dead_code)]
            pub fn $limit(&self) -> &str {
                &self.$limit
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
        minimum: String,
    }
);

struct_numeric_range_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "the number must be `<= {}`."]
    pub struct MaximumErrorParams {
        maximum: String,
    }
);

struct_numeric_range_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "the number must be `> {}`."]
    pub struct ExclusiveMinimumErrorParams {
        exclusive_minimum: String,
    }
);

struct_numeric_range_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "the number must be `< {}`."]
    pub struct ExclusiveMaximumErrorParams {
        exclusive_maximum: String,
    }
);
