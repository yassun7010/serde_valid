use crate::error::ToDefaultMessage;

/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/string.html#length>
macro_rules! struct_string_length_params {
    ($Param:tt, $limit:tt, $message:tt) => {
        #[derive(Debug, serde::Serialize)]
        pub struct $Param {
            value: String,
            $limit: usize,
        }

        impl $Param {
            pub fn new<T>(value: T, $limit: usize) -> Self
            where
                T: PartialOrd + PartialEq + std::fmt::Debug,
            {
                Self {
                    value: format!("{:?}", value),
                    $limit,
                }
            }

            #[allow(dead_code)]
            pub fn value(&self) -> &str {
                &self.value
            }

            #[allow(dead_code)]
            pub fn length(&self) -> usize {
                self.value.len()
            }

            #[allow(dead_code)]
            pub fn $limit(&self) -> usize {
                self.$limit
            }
        }

        impl ToDefaultMessage for $Param {
            fn to_default_message(&self) -> String {
                format!($message, self.$limit)
            }
        }
    };
}

struct_string_length_params!(
    MinLengthErrorParams,
    min_length,
    "the length of the value must be `>= {}`."
);
struct_string_length_params!(
    MaxLengthErrorParams,
    max_length,
    "the length of the value must be `<= {}`."
);
