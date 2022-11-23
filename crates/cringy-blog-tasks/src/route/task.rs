//! Task routes of the microservice.

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use uuid::Uuid;

use crate::data::model::{CreateTask, UpdateTask};
use crate::data::repository::DynTaskRepository;
use crate::utils::AppError;

/// All routers of the module merged in one.
pub fn all_merged() -> Router<DynTaskRepository> {
    Router::new()
        .merge(get_all())
        .merge(get_one())
        .merge(create_one())
        .merge(update_one())
        .merge(delete_one())
}

/// Router for `GET /task/all`.
pub fn get_all() -> Router<DynTaskRepository> {
    async fn handler(
        State(task_repo): State<DynTaskRepository>,
    ) -> Result<impl IntoResponse, AppError> {
        let all = task_repo.get_all().await?;
        Ok(Json(all))
    }

    Router::new().route("/task/all", get(handler))
}

/// Router for `GET /task/{id}`.
pub fn get_one() -> Router<DynTaskRepository> {
    async fn handler(
        State(task_repo): State<DynTaskRepository>,
        Path(id): Path<Uuid>,
    ) -> Result<impl IntoResponse, AppError> {
        let task = task_repo.get_one(id).await?;
        Ok(Json(task))
    }

    Router::new().route("/task/:id", get(handler))
}

/// Router for `POST /task/new`.
pub fn create_one() -> Router<DynTaskRepository> {
    async fn handler(
        State(task_repo): State<DynTaskRepository>,
        Json(create): Json<CreateTask>,
    ) -> Result<impl IntoResponse, AppError> {
        let task = task_repo.create_one(create).await?;
        Ok(Json(task))
    }

    Router::new().route("/task/new", post(handler))
}

/// Router for `POST /task/{id}`.
pub fn update_one() -> Router<DynTaskRepository> {
    async fn handler(
        State(task_repo): State<DynTaskRepository>,
        Path(id): Path<Uuid>,
        Json(update): Json<UpdateTask>,
    ) -> Result<impl IntoResponse, AppError> {
        let task = task_repo.update_one(id, update).await?;
        Ok(Json(task))
    }

    Router::new().route("/task/:id", post(handler))
}

/// Router for `DELETE /task/{id}`.
pub fn delete_one() -> Router<DynTaskRepository> {
    async fn handler(
        State(task_repo): State<DynTaskRepository>,
        Path(id): Path<Uuid>,
    ) -> Result<impl IntoResponse, AppError> {
        let task = task_repo.delete_one(id).await?;
        Ok(Json(task))
    }

    Router::new().route("/task/:id", delete(handler))
}
