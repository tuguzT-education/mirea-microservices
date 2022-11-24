//! blog routes of the microservice.

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use uuid::Uuid;

use crate::data::model::{CreateBlog, UpdateBlog};
use crate::data::repository::DynBlogRepository;
use crate::utils::AppError;

/// All routers of the module merged in one.
pub fn all_merged() -> Router<DynBlogRepository> {
    Router::new()
        .merge(get_all())
        .merge(get_one())
        .merge(create_one())
        .merge(update_one())
        .merge(delete_one())
}

/// Router for `GET /blog/all`.
pub fn get_all() -> Router<DynBlogRepository> {
    async fn handler(
        State(blog_repo): State<DynBlogRepository>,
    ) -> Result<impl IntoResponse, AppError> {
        let all = blog_repo.get_all().await?;
        Ok(Json(all))
    }

    Router::new().route("/blog/all", get(handler))
}

/// Router for `GET /blog/{id}`.
pub fn get_one() -> Router<DynBlogRepository> {
    async fn handler(
        State(blog_repo): State<DynBlogRepository>,
        Path(id): Path<Uuid>,
    ) -> Result<impl IntoResponse, AppError> {
        let blog = blog_repo.get_one(id).await?;
        Ok(Json(blog))
    }

    Router::new().route("/blog/:id", get(handler))
}

/// Router for `POST /blog/new`.
pub fn create_one() -> Router<DynBlogRepository> {
    async fn handler(
        State(blog_repo): State<DynBlogRepository>,
        Json(create): Json<CreateBlog>,
    ) -> Result<impl IntoResponse, AppError> {
        let blog = blog_repo.create_one(create).await?;
        Ok(Json(blog))
    }

    Router::new().route("/blog/new", post(handler))
}

/// Router for `POST /blog/{id}`.
pub fn update_one() -> Router<DynBlogRepository> {
    async fn handler(
        State(blog_repo): State<DynBlogRepository>,
        Path(id): Path<Uuid>,
        Json(update): Json<UpdateBlog>,
    ) -> Result<impl IntoResponse, AppError> {
        let blog = blog_repo.update_one(id, update).await?;
        Ok(Json(blog))
    }

    Router::new().route("/blog/:id", post(handler))
}

/// Router for `DELETE /blog/{id}`.
pub fn delete_one() -> Router<DynBlogRepository> {
    async fn handler(
        State(blog_repo): State<DynBlogRepository>,
        Path(id): Path<Uuid>,
    ) -> Result<impl IntoResponse, AppError> {
        let blog = blog_repo.delete_one(id).await?;
        Ok(Json(blog))
    }

    Router::new().route("/blog/:id", delete(handler))
}
