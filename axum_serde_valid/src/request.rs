use std::any::type_name;

use axum::{extract::FromRequest, BoxError};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde_json::Value;
use serde_valid::Validate;

use crate::context::SchemaContext;

pub async fn from_request<S, B, T>(
    req: axum::http::Request<B>,
    state: &S,
) -> Result<T, crate::Rejection>
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