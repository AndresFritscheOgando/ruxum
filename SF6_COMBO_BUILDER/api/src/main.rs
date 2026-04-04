mod config;
mod errors;
mod handlers;
mod router;
mod db;
mod auth;

use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub struct AppState {
    pub config: config::AppConfig,
    pub db: db::Db,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();
    let cfg = config::AppConfig::load()?;
    let addr = format!("{}:{}", cfg.host, cfg.port);
    let db = db::connect(cfg.database_url.as_deref().unwrap()).await?;

    let state = Arc::new(AppState { config: cfg, db });
    let app = router::app_router(state);

    tracing::info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
