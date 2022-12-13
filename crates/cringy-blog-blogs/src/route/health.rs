//! Health endpoints of the service.

use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

/// Health endpoint of the service.
pub fn health() -> Router {
    async fn handler() -> impl IntoResponse {
        "Healthy"
    }

    Router::new().route("/health", get(handler))
}
