use crate::validation::error::ToDefaultMessage;

/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/array.html#length>
macro_rules! struct_array_length_params {
    ($Param:tt, $field:tt, $message:tt) => {
        #[derive(Debug)]
        pub struct $Param {
            items: Vec<String>,
            $field: usize,
        }

        impl $Param {
            pub fn new<T>(items: &[T], $field: usize) -> Self
            where
                T: std::fmt::Debug,
            {
                Self {
                    items: items.iter().map(|i| format!("{:?}", i)).collect(),
                    $field,
                }
            }

            #[allow(dead_code)]
            pub fn items(&self) -> &Vec<String> {
                &self.items
            }

            #[allow(dead_code)]
            pub fn $field(&self) -> usize {
                self.$field
            }
        }

        impl ToDefaultMessage for $Param {
            fn to_default_message(&self) -> String {
                format!($message, self.$field,)
            }
        }
    };
}

struct_array_length_params!(
    MinItemsParams,
    min_items,
    "the length of the items must be `>= {}`."
);
struct_array_length_params!(
    MaxItemsParams,
    max_items,
    "the length of the items must be `<= {}`."
);
