use crate::error::ToDefaultMessage;

/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/array.html#length>
macro_rules! struct_array_length_params {
    ($Param:tt, $limit:tt, $message:tt) => {
        #[derive(Debug)]
        pub struct $Param {
            items: Vec<String>,
            $limit: usize,
        }

        impl $Param {
            pub fn new<T>(items: &[T], $limit: usize) -> Self
            where
                T: std::fmt::Debug,
            {
                Self {
                    items: items.iter().map(|i| format!("{:?}", i)).collect(),
                    $limit,
                }
            }

            #[allow(dead_code)]
            pub fn items(&self) -> &Vec<String> {
                &self.items
            }

            #[allow(dead_code)]
            pub fn $limit(&self) -> usize {
                self.$limit
            }
        }

        impl ToDefaultMessage for $Param {
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
