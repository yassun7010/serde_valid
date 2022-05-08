use crate::error::ToDefaultMessage;

/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/array.html#length>
macro_rules! struct_array_length_params {
    ($Params:tt, $limit:tt, $message:tt) => {
        #[derive(Debug)]
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
                format!($message, self.$limit,)
            }
        }
    };
}

struct_array_length_params!(
    MinItemsErrorParams,
    min_items,
    "the length of the items must be `>= {}`."
);
struct_array_length_params!(
    MaxItemsErrorParams,
    max_items,
    "the length of the items must be `<= {}`."
);
