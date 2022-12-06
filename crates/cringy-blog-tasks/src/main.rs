//! Simple microservice for university project.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![forbid(unsafe_code)]

use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Context;
use axum::{Router, Server};
use chrono::{Duration, Utc};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

use self::data::model::CreateTask;
use self::data::repository::{DynTaskRepository, LocalTaskRepository};
use self::route::task;

pub mod data;
pub mod route;
pub mod utils;

mod schema;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if cfg!(debug_assertions) {
        dotenv::dotenv().with_context(|| ".env file not found")?;
    }

    let log_directives = std::env::var("RUST_LOG").expect("RUST_LOG must be set");
    let gelf_addr = std::env::var("GELF_ADDR").expect("GELF_ADDR must be set");
    let (gelf, mut conn) = tracing_gelf::Logger::builder()
        .connect_udp(gelf_addr)
        .with_context(|| "failed to initialize GELF logger")?;
    tokio::spawn(async move { conn.connect().await });
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(log_directives))
        .with(gelf)
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .with_context(|| "failed to establish connection to database")?;
    if cfg!(not(debug_assertions)) {
        const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

        let connection = &mut pool.get()?;
        connection.run_pending_migrations(MIGRATIONS).unwrap();
    }

    let task_repo = Arc::new(LocalTaskRepository::new(pool)) as DynTaskRepository;
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

    let addr = &SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);
    Server::bind(addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c")
}
