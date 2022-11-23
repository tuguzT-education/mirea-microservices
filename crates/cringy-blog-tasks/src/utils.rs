//! Utilities for microservice.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use derive_more::{Display, Error, From};
use serde_json::json;

use crate::data::repository::TaskRepoError;

/// Top level application error type.
#[derive(Debug, Display, From, Error)]
pub enum AppError {
    /// Task repository error variant.
    TaskRepo(TaskRepoError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::TaskRepo(err) => match err {
                TaskRepoError::ExistsById => StatusCode::CONFLICT,
                TaskRepoError::NoTaskById => StatusCode::NOT_FOUND,
            },
        };
        let body = Json(json!({ "error": self.to_string() }));
        (status, body).into_response()
    }
}
