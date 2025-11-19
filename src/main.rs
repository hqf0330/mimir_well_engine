//! Mimir Well Engine - Main entry point
//!
//! A semantic layer engine service built with Rust and Axum.

use mimir_well_engine::api::router;
use mimir_well_engine::config::Settings;
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Load configuration
    let settings = Settings::default();
    let addr = SocketAddr::from(([0, 0, 0, 0], settings.server.port));

    info!("Starting Mimir Well Engine server on {}", addr);

    // Build application with routes
    let app = router();

    // Start server
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
