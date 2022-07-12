use crate::error::ToDefaultMessage;

/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/array.html#length>
macro_rules! struct_array_length_params {
    ($ErrorType:ident, $message:tt) => {
        paste::paste! {
            #[derive(Debug, Clone)]
            pub struct [<$ErrorType ErrorParams>] {
                pub [<$ErrorType:snake>]: usize,
            }

            impl [<$ErrorType ErrorParams>] {
                pub fn new([<$ErrorType:snake>]: usize) -> Self {
                    Self { [<$ErrorType:snake>] }
                }
            }

            impl ToDefaultMessage for [<$ErrorType ErrorParams>] {
                fn to_default_message(&self) -> String {
                    format!($message, self.[<$ErrorType:snake>],)
                }
            }
        }
    };
}

struct_array_length_params!(MinItems, "the length of the items must be `>= {}`.");
struct_array_length_params!(MaxItems, "the length of the items must be `<= {}`.");
