[package]
name = "{{project_name}}"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = { version = "0.7", features = ["macros"] }
tokio = { version = "1", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
dotenvy = "0.15"
config = "0.14"
anyhow = "1"
thiserror = "1"
{{db_dependencies}}
{{auth_dependencies}}
{{openapi_dependencies}}
