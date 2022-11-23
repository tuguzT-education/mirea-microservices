//! Task routes of the microservice.

use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::Router;

/// All routers of the module merged in one.
pub fn all_merged() -> Router {
    Router::new()
        .merge(get_all())
        .merge(get_one())
        .merge(create_one())
        .merge(update_one())
        .merge(delete_one())
}

/// Router for `GET /task/all`.
pub fn get_all() -> Router {
    async fn handler() -> impl IntoResponse {
        "All users are here!"
    }

    Router::new().route("/task/all", get(handler))
}

/// Router for `GET /task/{id}`.
pub fn get_one() -> Router {
    async fn handler(Path(id): Path<String>) -> impl IntoResponse {
        format!("Get user by id {}", id)
    }

    Router::new().route("/task/:id", get(handler))
}

/// Router for `POST /task/new`.
pub fn create_one() -> Router {
    async fn handler() -> impl IntoResponse {
        "Create user here!"
    }

    Router::new().route("/task/new", post(handler))
}

/// Router for `POST /task/{id}`.
pub fn update_one() -> Router {
    async fn handler(Path(id): Path<String>) -> impl IntoResponse {
        format!("Update user by id {}", id)
    }

    Router::new().route("/task/:id", post(handler))
}

/// Router for `DELETE /task/{id}`.
pub fn delete_one() -> Router {
    async fn handler(Path(id): Path<String>) -> impl IntoResponse {
        format!("Delete user by id {}", id)
    }

    Router::new().route("/task/:id", delete(handler))
}
