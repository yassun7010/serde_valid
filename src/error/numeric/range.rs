use crate::error::ToDefaultMessage;

macro_rules! struct_numeric_range_params {
    ($Param:tt, $field:tt, $message:tt) => {
        #[derive(Debug)]
        pub struct $Param {
            value: String,
            $field: String,
        }

        impl $Param {
            pub fn new<T>(value: T, $field: T) -> Self
            where
                T: PartialOrd + PartialEq + ToString,
            {
                Self {
                    value: value.to_string(),
                    $field: $field.to_string(),
                }
            }

            #[allow(dead_code)]
            pub fn value(&self) -> &str {
                &self.value
            }

            #[allow(dead_code)]
            pub fn $field(&self) -> &str {
                &self.$field
            }
        }

        impl ToDefaultMessage for $Param {
            fn to_default_message(&self) -> String {
                format!($message, self.$field)
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
