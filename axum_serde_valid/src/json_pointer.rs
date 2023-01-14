use std::ops::Deref;

use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct JsonPointer(pub String);

impl Deref for JsonPointer {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "aide")]
mod jsonschema {
    use schemars::{
        gen::SchemaGenerator,
        schema::{InstanceType, Schema, SchemaObject},
        JsonSchema,
    };

    use super::JsonPointer;

    impl JsonSchema for JsonPointer {
        fn schema_name() -> String {
            "JsonPointer".to_owned()
        }

        fn json_schema(_: &mut SchemaGenerator) -> Schema {
            SchemaObject {
                instance_type: Some(InstanceType::String.into()),
                format: None,
                ..Default::default()
            }
            .into()
        }
    }
}
