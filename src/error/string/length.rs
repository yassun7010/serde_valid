use crate::error::ToDefaultMessage;

/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/string.html#length>
macro_rules! struct_string_length_params {
    ($Params:tt, $limit:tt, $message:tt) => {
        #[derive(Debug, Clone, serde::Serialize)]
        pub struct $Params {
            $limit: usize,
        }

        impl $Params {
            pub fn new($limit: usize) -> Self {
                Self { $limit }
            }

            #[allow(dead_code)]
            pub fn $limit(&self) -> usize {
                self.$limit
            }
        }

        impl ToDefaultMessage for $Params {
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
