use std::collections::VecDeque;

use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse};
use jsonschema::output::{ErrorDescription, OutputUnit};
use schemars::JsonSchema;
use serde::Serialize;
use serde_valid::flatten::IntoFlat;

use crate::json_pointer::JsonPointer;

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
pub enum JsonErrorResponse {
    FormatError(JsonFormatErrorResponse),
    ValidationError(JsonSchemaErrorResponse),
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct JsonFormatErrorResponse {
    error: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct JsonSchemaErrorResponse {
    errors: Vec<Error>,
}

/// The response that is returned by default.
#[derive(Debug, Serialize, JsonSchema)]
pub struct Error {
    pub path: JsonPointer,
    pub message: String,
}

impl From<Rejection> for JsonErrorResponse {
    fn from(rejection: Rejection) -> Self {
        match rejection {
            Rejection::Json(v) => Self::FormatError(JsonFormatErrorResponse {
                error: v.to_string(),
            }),
            Rejection::Serde(_) => Self::FormatError(JsonFormatErrorResponse {
                error: "invalid request".to_string(),
            }),
            Rejection::Schema(errors) => Self::ValidationError(JsonSchemaErrorResponse {
                errors: errors
                    .into_iter()
                    .map(|error| Error {
                        path: JsonPointer(error.instance_location().to_owned()),
                        message: error.error_description().to_string(),
                    })
                    .collect::<Vec<_>>(),
            }),
            Rejection::SerdeValid(errors) => Self::ValidationError(JsonSchemaErrorResponse {
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
        match JsonErrorResponse::from(self) {
            JsonErrorResponse::FormatError(error) => {
                let mut response = axum::Json(error).into_response();
                *response.status_mut() = StatusCode::BAD_REQUEST;
                response
            }
            JsonErrorResponse::ValidationError(error) => {
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
            axum::Json::<JsonErrorResponse>::operation_response(ctx, operation)
        }

        fn inferred_responses(
            ctx: &mut aide::gen::GenContext,
            operation: &mut aide::openapi::Operation,
        ) -> Vec<(Option<u16>, aide::openapi::Response)> {
            let mut responses = vec![];

            if let Some(response) =
                axum::Json::<JsonFormatErrorResponse>::operation_response(ctx, operation)
            {
                responses.push((Some(StatusCode::BAD_REQUEST.into()), response));
            }
            if let Some(response) =
                axum::Json::<JsonSchemaErrorResponse>::operation_response(ctx, operation)
            {
                responses.push((Some(StatusCode::UNPROCESSABLE_ENTITY.into()), response));
            }

            responses
        }
    }
}
