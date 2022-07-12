use crate::error::ToDefaultMessage;

/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/string.html#length>
macro_rules! struct_string_length_params {
    (
        #[derive(Debug, Clone)]
        #[default_message=$default_message:literal]
        pub struct $ErrorParams:ident {
            $limit:ident: usize,
        }
    ) => {
        #[derive(Debug, Clone)]
        pub struct $ErrorParams {
            $limit: usize,
        }

        impl $ErrorParams {
            pub fn new($limit: usize) -> Self {
                Self { $limit }
            }

            #[allow(dead_code)]
            pub fn $limit(&self) -> usize {
                self.$limit
            }
        }

        impl ToDefaultMessage for $ErrorParams {
            fn to_default_message(&self) -> String {
                format!($default_message, self.$limit)
            }
        }
    };
}

struct_string_length_params!(
    #[derive(Debug, Clone)]
    #[default_message = "the length of the value must be `>= {}`."]
    pub struct MinLengthErrorParams {
        min_length: usize,
    }
);
struct_string_length_params!(
    #[derive(Debug, Clone)]
    #[default_message = "the length of the value must be `<= {}`."]
    pub struct MaxLengthErrorParams {
        max_length: usize,
    }
);
