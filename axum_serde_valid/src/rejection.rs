use std::any::type_name;
use std::collections::VecDeque;

use axum::{
    extract::{rejection::JsonRejection, FromRequest},
    http::{Request, StatusCode},
    response::IntoResponse,
    BoxError,
};
use jsonschema::{
    output::{ErrorDescription, OutputUnit},
    paths::JSONPointer,
};
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use serde_valid::{flatten::IntoFlat, Validate};

use crate::context::SchemaContext;

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

impl Rejection {
    pub async fn from_value<S, B, T>(req: Request<B>, state: &S) -> Result<T, Rejection>
    where
        B: http_body::Body + Send + 'static,
        B::Data: Send,
        B::Error: Into<BoxError>,
        S: Send + Sync,
        T: DeserializeOwned + Validate + JsonSchema + 'static,
    {
        let value: Value = match axum::Json::from_request(req, state).await {
            Ok(j) => j.0,
            Err(error) => Err(crate::Rejection::Json(error))?,
        };

        SchemaContext::validate::<T>(&value).map_err(crate::Rejection::Schema)?;

        match serde_json::from_value::<T>(value) {
            Ok(v) => {
                v.validate().map_err(crate::Rejection::SerdeValid)?;

                Ok(v)
            }
            Err(error) => {
                tracing::error!(
                    %error,
                    type_name = type_name::<T>(),
                    "schema validation passed but serde failed"
                );
                Err(crate::Rejection::Serde(error))
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    errors: Vec<Error>,
}

/// The response that is returned by default.
#[derive(Debug, Serialize)]
pub struct Error {
    pub path: JSONPointer,
    pub message: String,
}

impl From<Rejection> for ErrorResponse {
    fn from(rejection: Rejection) -> Self {
        match rejection {
            Rejection::Json(v) => Self {
                errors: vec![Error {
                    path: JSONPointer::default(),
                    message: v.to_string(),
                }],
            },
            Rejection::Serde(_) => Self {
                errors: vec![Error {
                    path: JSONPointer::default(),
                    message: "invalid request".to_string(),
                }],
            },
            Rejection::Schema(errors) => Self {
                errors: errors
                    .into_iter()
                    .map(|error| Error {
                        path: error.instance_location().to_owned(),
                        message: error.error_description().to_string(),
                    })
                    .collect::<Vec<_>>(),
            },
            Rejection::SerdeValid(errors) => Self {
                errors: errors
                    .into_flat()
                    .into_iter()
                    .map(|error| Error {
                        path: error.path,
                        message: error.message,
                    })
                    .collect::<Vec<_>>(),
            },
        }
    }
}

impl IntoResponse for Rejection {
    fn into_response(self) -> axum::response::Response {
        let mut res = axum::Json(ErrorResponse::from(self)).into_response();
        *res.status_mut() = StatusCode::BAD_REQUEST;
        res
    }
}
