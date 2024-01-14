use std::any::type_name;

use axum::extract::FromRequest;
use serde_json::Value;

pub async fn from_request<S, T>(
    req: axum::extract::Request,
    state: &S,
) -> Result<T, crate::rejection::Rejection>
where
    S: crate::traits::state::State,
    T: crate::traits::validated::Deserialize + 'static,
{
    let value: Value = match axum::Json::from_request(req, state).await {
        Ok(j) => j.0,
        Err(error) => Err(crate::rejection::Rejection::Json(error))?,
    };

    #[cfg(feature = "jsonschema")]
    {
        crate::jsonschema::SchemaContext::validate::<T>(&value)
            .map_err(crate::rejection::Rejection::Jsonschema)?;
    }

    match serde_json::from_value::<T>(value) {
        Ok(v) => {
            v.validate()
                .map_err(crate::rejection::Rejection::SerdeValid)?;

            Ok(v)
        }
        Err(error) => {
            tracing::error!(
                %error,
                type_name = type_name::<T>(),
                "schema validation passed but serde failed"
            );
            Err(crate::rejection::Rejection::SerdeJson(error))
        }
    }
}
