#![cfg_attr(docsrs, feature(doc_auto_cfg))]
//! A simple crate provides a drop-in replacement for [`axum::Json`]
//! that uses [jsonschema](https://docs.rs/jsonschema/latest/jsonschema/) to validate requests schemas
//! generated via [schemars](https://docs.rs/schemars/latest/schemars/).
//!
//! You might want to do this in order to provide a better
//! experience for your clients and not leak serde's error messages.
//!
//! All schemas are cached in a thread-local storage for
//! the life of the application (or thread).
//!
//! # Features
//!
//! - aide: support for [aide](https://docs.rs/aide/latest/aide/)

use async_trait::async_trait;
use axum::extract::Request;
use axum::{extract::FromRequest, response::IntoResponse};
use serde::Serialize;
use std::ops::Deref;

/// Wrapper type over [`axum::Json`] that validates
/// requests and responds with a more helpful validation
/// message.
pub struct Json<T>(pub T);

impl<T> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for Json<T> {
    fn from(data: T) -> Self {
        Json(data)
    }
}

#[async_trait]
impl<T, S> FromRequest<S> for Json<T>
where
    T: crate::traits::validated::Deserialize + 'static,
    S: crate::traits::state::State,
{
    type Rejection = crate::rejection::Rejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        crate::request::from_request::<_, T>(req, state)
            .await
            .map(Json)
    }
}

impl<T> IntoResponse for Json<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}

#[cfg(feature = "aide")]
mod impl_aide {
    use super::*;

    impl<T> aide::OperationInput for Json<T>
    where
        T: schemars::JsonSchema,
    {
        fn operation_input(
            ctx: &mut aide::gen::GenContext,
            operation: &mut aide::openapi::Operation,
        ) {
            axum::Json::<T>::operation_input(ctx, operation);
        }
    }

    impl<T> aide::OperationOutput for Json<T>
    where
        T: schemars::JsonSchema,
    {
        type Inner = <axum::Json<T> as aide::OperationOutput>::Inner;

        fn operation_response(
            ctx: &mut aide::gen::GenContext,
            op: &mut aide::openapi::Operation,
        ) -> Option<aide::openapi::Response> {
            axum::Json::<T>::operation_response(ctx, op)
        }

        fn inferred_responses(
            ctx: &mut aide::gen::GenContext,
            operation: &mut aide::openapi::Operation,
        ) -> Vec<(Option<u16>, aide::openapi::Response)> {
            axum::Json::<T>::inferred_responses(ctx, operation)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Json;
    use axum::http::StatusCode;
    use axum::{
        body::Body,
        http::{self, Request},
    };
    use serde::Deserialize;
    use serde_json::json;
    use serde_valid::Validate;
    use tower::ServiceExt;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[cfg(all(not(feature = "jsonschema"), not(feature = "aide")))]
    #[tokio::test]
    async fn test_json() -> TestResult {
        use axum::{routing::post, Router};

        #[derive(Deserialize, Validate)]
        struct User {
            #[validate(max_length = 3)]
            name: String,
        }

        let app = Router::new().route("/json", post(|_user: Json<User>| async move { "hello" }));

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/json")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_vec(&json!({"name": "taro"}))?))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(
                &axum::body::to_bytes(response.into_body(), 1_000_000).await?,
            )?,
            json!({"errors": [
                {
                    "error": "The length of the value must be `<= 3`.",
                    "instance_location": "/name",
                    "keyword_location": null
                }
            ]})
        );

        Ok(())
    }

    #[cfg(feature = "jsonschema")]
    #[tokio::test]
    async fn test_json_with_jsonschema() -> TestResult {
        use axum::{routing::post, Router};

        #[derive(Deserialize, Validate, schemars::JsonSchema)]
        struct User {
            #[validate(max_length = 3)]
            name: String,
        }

        let app = Router::new().route("/json", post(|_user: Json<User>| async move { "hello" }));

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/json")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_vec(&json!({"name": "taro"}))?))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(
                &axum::body::to_bytes(response.into_body(), 1_000_000).await?
            )?,
            json!({"errors": [
                {
                    "error": "The length of the value must be `<= 3`.",
                    "instance_location": "/name",
                    "keyword_location": null
                }
            ]})
        );

        Ok(())
    }

    #[cfg(feature = "aide")]
    #[tokio::test]
    async fn test_json_with_aide() -> TestResult {
        use aide::axum::{routing::post, ApiRouter};

        #[derive(Deserialize, Validate, schemars::JsonSchema)]
        struct User {
            #[validate(max_length = 3)]
            name: String,
        }

        let app = ApiRouter::new().route("/json", post(|_user: Json<User>| async move { "hello" }));

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/json")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_vec(&json!({"name": "taro"}))?))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(
                &axum::body::to_bytes(response.into_body(), 1_000_000).await?,
            )?,
            json!({"errors": [
                {
                    "error": "The length of the value must be `<= 3`.",
                    "instance_location": "/name",
                    "keyword_location": null
                }
            ]})
        );

        Ok(())
    }
}
