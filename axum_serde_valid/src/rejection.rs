use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use serde_valid::flatten::IntoFlat;

use crate::json_pointer::JsonPointer;

/// Rejection for [`axum::Json`].
#[derive(Debug)]
pub enum Rejection {
    /// A rejection returned by [`axum::Json`].
    Json(JsonRejection),
    /// A serde json error.
    SerdeJson(serde_json::Error),
    /// A serde url encoded error.
    SerdeUrlEncoded(serde_urlencoded::de::Error),
    /// A serde_valid validation error.
    SerdeValid(serde_valid::validation::Errors),
    #[cfg(feature = "jsonschema")]
    /// A schema validation error.
    Jsonschema(
        std::collections::VecDeque<
            jsonschema::output::OutputUnit<jsonschema::output::ErrorDescription>,
        >,
    ),
}

#[cfg(not(feature = "aide"))]
#[derive(Debug, Serialize)]
pub enum JsonErrorResponse {
    FormatError(String),
    ValidationError(JsonSchemaErrorResponse),
}

#[cfg(feature = "aide")]
#[derive(Debug, Serialize, schemars::JsonSchema)]
pub enum JsonErrorResponse {
    FormatError(String),
    ValidationError(JsonSchemaErrorResponse),
}

#[cfg(not(feature = "aide"))]
#[derive(Debug, Serialize)]
pub struct JsonSchemaErrorResponse {
    errors: Vec<Error>,
}

#[cfg(feature = "aide")]
#[derive(Debug, Serialize, schemars::JsonSchema)]
pub struct JsonSchemaErrorResponse {
    errors: Vec<Error>,
}

#[cfg(not(feature = "aide"))]
#[derive(Debug, Serialize)]
pub struct Error {
    pub error: String,
    pub instance_location: JsonPointer,
    pub keyword_location: Option<JsonPointer>,
}

#[cfg(feature = "aide")]
#[derive(Debug, Serialize, schemars::JsonSchema)]
pub struct Error {
    pub error: String,
    pub instance_location: JsonPointer,
    pub keyword_location: Option<JsonPointer>,
}

impl From<Rejection> for JsonErrorResponse {
    fn from(rejection: Rejection) -> Self {
        match rejection {
            Rejection::Json(error) => Self::FormatError(error.to_string()),
            Rejection::SerdeJson(error) => Self::FormatError(error.to_string()),
            Rejection::SerdeUrlEncoded(error) => Self::FormatError(error.to_string()),
            Rejection::SerdeValid(errors) => {
                let iter = errors.into_flat().into_iter().map(|err| Error {
                    error: err.error,
                    instance_location: JsonPointer(err.instance_location.to_string()),
                    keyword_location: None,
                });

                Self::ValidationError(JsonSchemaErrorResponse {
                    errors: iter.collect::<Vec<_>>(),
                })
            }
            #[cfg(feature = "jsonschema")]
            Rejection::Jsonschema(errors) => Self::ValidationError(JsonSchemaErrorResponse {
                errors: errors
                    .into_iter()
                    .map(|err| Error {
                        error: err.error_description().to_string(),
                        instance_location: JsonPointer(err.instance_location().to_string()),
                        keyword_location: Some(JsonPointer(err.keyword_location().to_string())),
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

            if let Some(response) = String::operation_response(ctx, operation) {
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
