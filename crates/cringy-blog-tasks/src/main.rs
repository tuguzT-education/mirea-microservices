//! Simple microservice for university project.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![forbid(unsafe_code)]

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{Router, Server};
use chrono::{Duration, Utc};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

use self::data::model::CreateTask;
use self::data::repository::{DynTaskRepository, InMemoryTaskRepository};
use self::route::task;

pub mod data;
pub mod route;
pub mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if dotenv::dotenv().is_err() {
        println!(".env file not found, server may panic unexpectedly");
    }
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "cringy_blog_tasks=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    let task_repo = Arc::new(InMemoryTaskRepository::default()) as DynTaskRepository;
    let new_task = CreateTask {
        blog_id: Uuid::new_v4(),
        name: "New task".to_string(),
        deadline: Some(Utc::now() + Duration::days(1)),
    };
    task_repo.create_one(new_task).await?;
    let app = Router::new()
        .merge(task::all_merged())
        .layer(TraceLayer::new_for_http())
        .with_state(task_repo);

    let addr = &SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {}", addr);
    Server::bind(addr).serve(app.into_make_service()).await?;
    Ok(())
}
