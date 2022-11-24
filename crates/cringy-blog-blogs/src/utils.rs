//! Utilities for microservice.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use derive_more::{Display, Error, From};
use serde_json::json;

use crate::data::repository::BlogRepoError;

/// Top level application error type.
#[derive(Debug, Display, From, Error)]
pub enum AppError {
    /// Blog repository error variant.
    BlogRepo(BlogRepoError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::BlogRepo(err) => match err {
                BlogRepoError::ExistsById => StatusCode::CONFLICT,
                BlogRepoError::NoBlogById => StatusCode::NOT_FOUND,
            },
        };
        let body = Json(json!({ "error": self.to_string() }));
        (status, body).into_response()
    }
}
