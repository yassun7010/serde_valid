#![cfg_attr(docsrs, feature(doc_auto_cfg))]
//! A simple crate provides a drop-in replacement for [`axum::extract::Query`]
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

use std::ops::Deref;

use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use serde::de::DeserializeOwned;

/// Wrapper type over [`axum::extract::Query`] that validates
/// requests with a more helpful validation
/// message.
pub struct Query<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for Query<T>
where
    T: DeserializeOwned + serde_valid::Validate,
    S: Send + Sync,
{
    type Rejection = crate::rejection::Rejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let query = parts.uri.query().unwrap_or_default();
        match serde_urlencoded::from_str::<T>(query) {
            Ok(v) => {
                v.validate()
                    .map_err(crate::rejection::Rejection::SerdeValid)?;

                Ok(Query(v))
            }
            Err(error) => Err(crate::rejection::Rejection::SerdeUrlEncoded(error)),
        }
    }
}

impl<T> Deref for Query<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for Query<T> {
    fn from(data: T) -> Self {
        Query(data)
    }
}

#[cfg(feature = "aide")]
mod impl_aide {
    use super::*;

    impl<T> aide::OperationInput for Query<T>
    where
        T: schemars::JsonSchema,
    {
        fn operation_input(
            ctx: &mut aide::gen::GenContext,
            operation: &mut aide::openapi::Operation,
        ) {
            axum::extract::Query::<T>::operation_input(ctx, operation);
        }
    }
}
