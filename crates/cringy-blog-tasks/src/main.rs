//! Simple microservice for university project.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![forbid(unsafe_code)]

use std::net::SocketAddr;

use axum::{routing::get, Router, Server};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use self::route::root;

mod route;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if dotenv::dotenv().is_err() {
        tracing::info!(".env file not found, server may panic unexpectedly");
    }
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "cringy_blog_tasks=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    let app = Router::new()
        .route("/", get(root))
        .layer(TraceLayer::new_for_http());

    let addr = &SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {}", addr);
    Server::bind(addr).serve(app.into_make_service()).await?;
    Ok(())
}
