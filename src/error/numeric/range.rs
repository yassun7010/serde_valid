use crate::error::ToDefaultMessage;

macro_rules! struct_numeric_range_params {
    ($Param:tt, $limit:tt, $message:tt) => {
        #[derive(Debug)]
        pub struct $Param {
            value: String,
            $limit: String,
        }

        impl $Param {
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

        impl ToDefaultMessage for $Param {
            fn to_default_message(&self) -> String {
                format!($message, self.$limit)
            }
        }
    };
}

struct_numeric_range_params!(MinimumParams, minimum, "the number must be `>= {}`.");
struct_numeric_range_params!(MaximumParams, maximum, "the number must be `<= {}`.");
struct_numeric_range_params!(
    ExclusiveMinimumParams,
    exclusive_minimum,
    "the number must be `> {}`."
);
struct_numeric_range_params!(
    ExclusiveMaximumParams,
    exclusive_maximum,
    "the number must be `< {}`."
);
