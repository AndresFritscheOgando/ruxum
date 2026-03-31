import { ScaffoldConfig } from "../types";

export function buildMainRs(config: ScaffoldConfig): string {
  const db = config.rust?.db;
  const auth = config.rust?.auth;
  const hasDb = db && db !== "none";

  const mods = [
    "mod config;",
    "mod errors;",
    "mod handlers;",
    "mod router;",
    hasDb ? "mod db;" : "// mod db;",
    auth ? "mod auth;" : "// mod auth;",
  ].join("\n");

  const stateFields = hasDb
    ? `    pub config: config::AppConfig,
    pub db: db::Db,`
    : `    pub config: config::AppConfig,`;

  const isDiesel = db && db.startsWith("diesel");
  const dbConnect = hasDb
    ? isDiesel
      ? `    db::run_migrations(cfg.database_url.as_deref().unwrap()).await?;\n    let db = db::connect(cfg.database_url.as_deref().unwrap()).await?;\n`
      : `    let db = db::connect(cfg.database_url.as_deref().unwrap()).await?;\n`
    : "";

  const stateInit = hasDb
    ? `    let state = Arc::new(AppState { config: cfg, db });`
    : `    let state = Arc::new(AppState { config: cfg });`;

  return `${mods}

use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub struct AppState {
${stateFields}
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
${dbConnect}
${stateInit}
    let app = router::app_router(state);

    tracing::info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
`;
}
