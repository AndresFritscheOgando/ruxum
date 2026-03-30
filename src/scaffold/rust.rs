use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::config::{DbChoice, RustConfig};

pub fn scaffold(dir: &Path, cfg: &RustConfig) -> Result<()> {
    let has_db = cfg.db != DbChoice::None;

    fs::create_dir_all(dir.join("src/handlers"))?;
    if has_db {
        fs::create_dir_all(dir.join("src/models"))?;
    }
    if cfg.auth {
        fs::create_dir_all(dir.join("src/auth"))?;
    }

    write(dir, "Cargo.toml", &cargo_toml(cfg))?;
    write(dir, "src/main.rs", &main_rs(cfg))?;
    write(dir, "src/router.rs", router_rs())?;
    write(dir, "src/config.rs", &config_rs(cfg))?;
    write(dir, "src/errors.rs", errors_rs())?;
    write(dir, "src/handlers/mod.rs", "pub mod health;\n")?;
    write(dir, "src/handlers/health.rs", health_rs())?;

    if has_db {
        let db_content = if cfg.db.is_sqlx() {
            db_sqlx_rs(&cfg.db)
        } else {
            db_seaorm_rs().to_string()
        };
        write(dir, "src/db.rs", &db_content)?;
        write(dir, "src/models/mod.rs", "// Add your database models here\n")?;
    }

    if cfg.auth {
        write(dir, "src/auth/mod.rs", "pub mod middleware;\n")?;
        write(dir, "src/auth/middleware.rs", auth_middleware_rs())?;
    }

    write(dir, ".env.example", &env_example(cfg))?;

    Ok(())
}

fn write(dir: &Path, path: &str, content: &str) -> Result<()> {
    let full = dir.join(path);
    if let Some(parent) = full.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(full, content)?;
    Ok(())
}

fn cargo_toml(cfg: &RustConfig) -> String {
    let mut s = format!(
        r#"[package]
name = "{{project_name}}"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = {{ version = "0.7", features = ["macros"] }}
tokio = {{ version = "1", features = ["full"] }}
tower = "0.4"
tower-http = {{ version = "0.5", features = ["cors", "trace"] }}
serde = {{ version = "1", features = ["derive"] }}
serde_json = "1"
tracing = "0.1"
tracing-subscriber = {{ version = "0.3", features = ["env-filter"] }}
dotenvy = "0.15"
config = "0.14"
anyhow = "1"
thiserror = "1"
"#
    );

    match &cfg.db {
        DbChoice::SqlxPostgres => s.push_str(
            r#"sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono", "macros"] }
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
"#,
        ),
        DbChoice::SqlxMysql => s.push_str(
            r#"sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "mysql", "uuid", "chrono", "macros"] }
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
"#,
        ),
        DbChoice::SqlxSqlite => s.push_str(
            r#"sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "sqlite", "uuid", "chrono", "macros"] }
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
"#,
        ),
        DbChoice::SeaormPostgres => s.push_str(
            r#"sea-orm = { version = "0.12", features = ["sqlx-postgres", "runtime-tokio-rustls", "macros"] }
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
"#,
        ),
        DbChoice::SeaormMysql => s.push_str(
            r#"sea-orm = { version = "0.12", features = ["sqlx-mysql", "runtime-tokio-rustls", "macros"] }
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
"#,
        ),
        DbChoice::None => {}
    }

    if cfg.auth {
        s.push_str(
            r#"jsonwebtoken = "9"
axum-extra = { version = "0.9", features = ["typed-header"] }
headers = "0.4"
"#,
        );
    }

    s
}

fn main_rs(cfg: &RustConfig) -> String {
    let has_db = cfg.db != DbChoice::None;

    let mods = [
        "mod config;",
        "mod errors;",
        "mod handlers;",
        "mod router;",
        if has_db { "mod db;" } else { "// mod db;" },
        if cfg.auth { "mod auth;" } else { "// mod auth;" },
    ]
    .join("\n");

    let state_fields = if has_db {
        "    pub config: config::AppConfig,\n    pub db: db::Db,"
    } else {
        "    pub config: config::AppConfig,"
    };

    let db_connect = if has_db {
        "    let db = db::connect(cfg.database_url.as_deref().unwrap()).await?;\n"
    } else {
        ""
    };

    let state_init = if has_db {
        "    let state = Arc::new(AppState { config: cfg, db });"
    } else {
        "    let state = Arc::new(AppState { config: cfg });"
    };

    format!(
        r#"{mods}

use std::sync::Arc;
use tracing_subscriber::{{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter}};

pub struct AppState {{
{state_fields}
}}

#[tokio::main]
async fn main() -> anyhow::Result<()> {{
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();
    let cfg = config::AppConfig::load()?;
    let addr = format!("{{}}:{{}}", cfg.host, cfg.port);
{db_connect}
{state_init}
    let app = router::app_router(state);

    tracing::info!("Listening on {{}}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}}
"#
    )
}

fn router_rs() -> &'static str {
    r#"use axum::{routing::get, Router};
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use crate::AppState;

pub fn app_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(crate::handlers::health::health_check))
        // If JWT selected, add a protected route example:
        // .route("/me", get(crate::handlers::user::me)
        //     .route_layer(axum::middleware::from_fn_with_state(
        //         state.clone(),
        //         crate::auth::middleware::require_auth,
        //     )))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
"#
}

fn config_rs(cfg: &RustConfig) -> String {
    let has_db = cfg.db != DbChoice::None;

    let mut fields = vec![
        "    pub host: String,".to_string(),
        "    pub port: u16,".to_string(),
    ];
    if has_db {
        fields.push("    pub database_url: Option<String>,".to_string());
    }
    if cfg.auth {
        fields.push("    pub jwt_secret: Option<String>,".to_string());
    }
    let fields = fields.join("\n");

    format!(
        r#"use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {{
{fields}
}}

impl AppConfig {{
    pub fn load() -> Result<Self> {{
        let cfg = config::Config::builder()
            .add_source(config::Environment::default())
            .set_default("host", "127.0.0.1")?
            .set_default("port", 3000)?
            .build()?;
        Ok(cfg.try_deserialize()?)
    }}
}}
"#
    )
}

fn errors_rs() -> &'static str {
    r#"use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found")]
    NotFound,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
"#
}

fn health_rs() -> &'static str {
    r#"use axum::Json;
use serde_json::{json, Value};

pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    }))
}
"#
}

fn db_sqlx_rs(db: &DbChoice) -> String {
    let (pool_options, pool_type, module) = match db {
        DbChoice::SqlxPostgres => ("PgPoolOptions", "PgPool", "sqlx::postgres"),
        DbChoice::SqlxMysql => ("MySqlPoolOptions", "MySqlPool", "sqlx::mysql"),
        DbChoice::SqlxSqlite => ("SqlitePoolOptions", "SqlitePool", "sqlx::sqlite"),
        _ => ("PgPoolOptions", "PgPool", "sqlx::postgres"),
    };

    format!(
        r#"use {module}::{pool_options};

pub type Db = sqlx::{pool_type};

pub async fn connect(url: &str) -> anyhow::Result<Db> {{
    let pool = {pool_options}::new()
        .max_connections(10)
        .connect(url)
        .await?;
    Ok(pool)
}}
"#
    )
}

fn db_seaorm_rs() -> &'static str {
    r#"use sea_orm::{Database, DatabaseConnection};

pub type Db = DatabaseConnection;

pub async fn connect(url: &str) -> anyhow::Result<Db> {
    let db = Database::connect(url).await?;
    Ok(db)
}
"#
}

fn auth_middleware_rs() -> &'static str {
    r#"use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::{errors::AppError, AppState};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn encode_token(secret: &str, sub: &str, exp: usize) -> Result<String, AppError> {
    encode(
        &Header::default(),
        &Claims { sub: sub.to_string(), exp },
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::InternalServerError(e.to_string()))
}

pub fn decode_token(secret: &str, token: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| AppError::Unauthorized)
}

pub struct AuthUser(pub Claims);

#[axum::async_trait]
impl FromRequestParts<Arc<AppState>> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| AppError::Unauthorized)?;

        let secret = state.config.jwt_secret.as_deref().unwrap_or("");
        let claims = decode_token(secret, bearer.token())?;
        Ok(AuthUser(claims))
    }
}
"#
}

fn env_example(cfg: &RustConfig) -> String {
    let mut s = "HOST=127.0.0.1\nPORT=3000\n".to_string();

    match &cfg.db {
        DbChoice::SqlxPostgres | DbChoice::SeaormPostgres => {
            s.push_str("DATABASE_URL=postgres://user:password@localhost/mydb\n");
        }
        DbChoice::SqlxMysql | DbChoice::SeaormMysql => {
            s.push_str("DATABASE_URL=mysql://user:password@localhost/mydb\n");
        }
        DbChoice::SqlxSqlite => {
            s.push_str("DATABASE_URL=sqlite://./dev.db\n");
        }
        DbChoice::None => {}
    }

    if cfg.auth {
        s.push_str("JWT_SECRET=changeme_use_a_long_random_string_in_production\n");
    }

    s
}
