use crate::error::ToDefaultMessage;

macro_rules! struct_numeric_range_params {
    ($Params:tt, $limit:tt, $message:tt) => {
        #[derive(Debug, Clone)]
        pub struct $Params {
            $limit: String,
        }

        impl $Params {
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

        impl ToDefaultMessage for $Params {
            fn to_default_message(&self) -> String {
                format!($message, self.$limit)
            }
        }
    };
}

struct_numeric_range_params!(MinimumErrorParams, minimum, "the number must be `>= {}`.");
struct_numeric_range_params!(MaximumErrorParams, maximum, "the number must be `<= {}`.");
struct_numeric_range_params!(
    ExclusiveMinimumErrorParams,
    exclusive_minimum,
    "the number must be `> {}`."
);
struct_numeric_range_params!(
    ExclusiveMaximumErrorParams,
    exclusive_maximum,
    "the number must be `< {}`."
);
