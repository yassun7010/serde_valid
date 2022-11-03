use std::{
    any::{type_name, TypeId},
    cell::RefCell,
    collections::{HashMap, VecDeque},
};

use jsonschema::{
    output::{BasicOutput, ErrorDescription, OutputUnit},
    JSONSchema,
};
use schemars::{
    gen::{SchemaGenerator, SchemaSettings},
    JsonSchema,
};
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use serde_valid::Validate;

thread_local! {
    static CONTEXT: RefCell<SchemaContext> = RefCell::new(SchemaContext::new());
}

pub(crate) struct SchemaContext {
    pub generator: SchemaGenerator,
    pub schemas: HashMap<TypeId, JSONSchema>,
}

impl SchemaContext {
    pub fn new() -> Self {
        Self {
            generator: SchemaSettings::draft07()
                .with(|g| g.inline_subschemas = true)
                .into_generator(),
            schemas: HashMap::default(),
        }
    }

    pub fn validate<T>(value: &Value) -> Result<(), VecDeque<OutputUnit<ErrorDescription>>>
    where
        T: DeserializeOwned + Validate + JsonSchema + 'static,
    {
        CONTEXT.with(|ctx| {
            let ctx = &mut *ctx.borrow_mut();
            let schema = ctx.schemas.entry(TypeId::of::<T>()).or_insert_with(|| {
                match jsonschema::JSONSchema::compile(
                    &serde_json::to_value(ctx.generator.root_schema_for::<T>()).unwrap(),
                ) {
                    Ok(s) => s,
                    Err(error) => {
                        tracing::error!(
                            %error,
                            type_name = type_name::<T>(),
                            "invalid JSON schema for type"
                        );
                        JSONSchema::compile(&Value::Object(Map::default())).unwrap()
                    }
                }
            });

            let out = schema.apply(value).basic();

            match out {
                BasicOutput::Valid(_) => Ok(()),
                BasicOutput::Invalid(v) => Err(v),
            }
        })
    }
}
