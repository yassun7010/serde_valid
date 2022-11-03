use std::{collections::VecDeque, ops::Deref};

use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse};
use jsonschema::output::{ErrorDescription, OutputUnit};
use schemars::{
    gen::SchemaGenerator,
    schema::{InstanceType, Schema, SchemaObject},
    JsonSchema,
};
use serde::Serialize;
use serde_valid::flatten::IntoFlat;

/// Rejection for [`Json`].
#[derive(Debug)]
pub enum Rejection {
    /// A rejection returned by [`axum::Json`].
    Json(JsonRejection),
    /// A serde error.
    Serde(serde_json::Error),
    /// A schema validation error.
    Schema(VecDeque<OutputUnit<ErrorDescription>>),
    /// A serde_valid validation error.
    SerdeValid(serde_valid::validation::Errors),
}

#[derive(Debug, Serialize, JsonSchema)]
pub enum ErrorResponse {
    FormatError(FormatErrorResponse),
    ValidationError(ValidationErrorResponse),
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct FormatErrorResponse {
    error: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct ValidationErrorResponse {
    errors: Vec<Error>,
}

#[derive(Debug, Default, Serialize)]
pub struct JsonPointer(jsonschema::paths::JSONPointer);

impl Deref for JsonPointer {
    type Target = jsonschema::paths::JSONPointer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

/// The response that is returned by default.
#[derive(Debug, Serialize, JsonSchema)]
pub struct Error {
    pub path: JsonPointer,
    pub message: String,
}

impl From<Rejection> for ErrorResponse {
    fn from(rejection: Rejection) -> Self {
        match rejection {
            Rejection::Json(v) => Self::FormatError(FormatErrorResponse {
                error: v.to_string(),
            }),
            Rejection::Serde(_) => Self::FormatError(FormatErrorResponse {
                error: "invalid request".to_string(),
            }),
            Rejection::Schema(errors) => Self::ValidationError(ValidationErrorResponse {
                errors: errors
                    .into_iter()
                    .map(|error| Error {
                        path: JsonPointer(error.instance_location().to_owned()),
                        message: error.error_description().to_string(),
                    })
                    .collect::<Vec<_>>(),
            }),
            Rejection::SerdeValid(errors) => Self::ValidationError(ValidationErrorResponse {
                errors: errors
                    .into_flat()
                    .into_iter()
                    .map(|error| Error {
                        path: JsonPointer(error.path),
                        message: error.message,
                    })
                    .collect::<Vec<_>>(),
            }),
        }
    }
}

impl IntoResponse for Rejection {
    fn into_response(self) -> axum::response::Response {
        match ErrorResponse::from(self) {
            ErrorResponse::FormatError(error) => {
                let mut response = axum::Json(error).into_response();
                *response.status_mut() = StatusCode::BAD_REQUEST;
                response
            }
            ErrorResponse::ValidationError(error) => {
                let mut response = axum::Json(error).into_response();
                *response.status_mut() = StatusCode::UNPROCESSABLE_ENTITY;
                response
            }
        }
    }
}

#[cfg(feature = "aide")]
mod impl_aide {
    use super::*;

    impl aide::OperationOutput for Rejection {
        type Inner = Self;

        fn operation_response(
            ctx: &mut aide::gen::GenContext,
            operation: &mut aide::openapi::Operation,
        ) -> Option<aide::openapi::Response> {
            axum::Json::<ErrorResponse>::operation_response(ctx, operation)
        }

        fn inferred_responses(
            ctx: &mut aide::gen::GenContext,
            operation: &mut aide::openapi::Operation,
        ) -> Vec<(Option<u16>, aide::openapi::Response)> {
            let mut responses = vec![];
            if let Some(res) = Self::operation_response(ctx, operation) {
                responses.push((Some(422), res));
            }

            responses
        }
    }
}
