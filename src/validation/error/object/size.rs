use crate::traits::Size;
use crate::validation::error::ToDefaultMessage;

macro_rules! struct_object_size_params {
    ($Param:tt, $field:tt, $message:tt) => {
        #[derive(Debug)]
        pub struct $Param {
            properties_size: usize,
            $field: usize,
        }

        impl $Param {
            pub fn new<T>(properties: &T, $field: usize) -> Self
            where
                T: Size,
            {
                Self {
                    properties_size: properties.size(),
                    $field,
                }
            }

            #[allow(dead_code)]
            pub fn properties_size(&self) -> usize {
                self.properties_size
            }

            #[allow(dead_code)]
            pub fn $field(&self) -> usize {
                self.$field
            }
        }

        impl ToDefaultMessage for $Param {
            fn to_default_message(&self) -> String {
                format!($message, self.$field)
            }
        }
    };
}

struct_object_size_params!(
    MinPropertiesParams,
    min_properties,
    "the size of the properties must be `>= {}`."
);
struct_object_size_params!(
    MaxPropertiesParams,
    max_properties,
    "the size of the properties must be `<= {}`."
);
