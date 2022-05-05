use crate::error::ToDefaultMessage;
use crate::traits::Size;

macro_rules! struct_object_size_params {
    ($Param:tt, $limit:tt, $message:tt) => {
        #[derive(Debug)]
        pub struct $Param {
            properties_size: usize,
            $limit: usize,
        }

        impl $Param {
            pub fn new<T>(properties: &T, $limit: usize) -> Self
            where
                T: Size,
            {
                Self {
                    properties_size: properties.size(),
                    $limit,
                }
            }

            #[allow(dead_code)]
            pub fn properties_size(&self) -> usize {
                self.properties_size
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
