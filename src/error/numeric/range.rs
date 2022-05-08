use crate::error::ToDefaultMessage;

macro_rules! struct_numeric_range_params {
    ($Params:tt, $limit:tt, $message:tt) => {
        #[derive(Debug)]
        pub struct $Params {
            value: String,
            $limit: String,
        }

        impl $Params {
            pub fn new<T>(value: T, $limit: T) -> Self
            where
                T: PartialOrd + PartialEq + ToString,
            {
                Self {
                    value: value.to_string(),
                    $limit: $limit.to_string(),
                }
            }

            #[allow(dead_code)]
            pub fn value(&self) -> &str {
                &self.value
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
