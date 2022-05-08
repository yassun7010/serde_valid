use crate::error::ToDefaultMessage;

macro_rules! struct_object_size_params {
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
                format!($message, self.$limit)
            }
        }
    };
}

struct_object_size_params!(
    MinPropertiesErrorParams,
    min_properties,
    "the size of the properties must be `>= {}`."
);
struct_object_size_params!(
    MaxPropertiesErrorParams,
    max_properties,
    "the size of the properties must be `<= {}`."
);
