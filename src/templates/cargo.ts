import { ScaffoldConfig } from "../types";

export function buildCargoToml(config: ScaffoldConfig): string {
  const { projectName, db, auth } = config;

  let toml = `[package]
name = "${projectName}"
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
`;

  if (db === "sqlx-postgres") {
    toml += `sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono", "macros"] }
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
`;
  } else if (db === "sqlx-mysql") {
    toml += `sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "mysql", "uuid", "chrono", "macros"] }
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
`;
  } else if (db === "sqlx-sqlite") {
    toml += `sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "sqlite", "uuid", "chrono", "macros"] }
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
`;
  } else if (db === "seaorm-postgres") {
    toml += `sea-orm = { version = "0.12", features = ["sqlx-postgres", "runtime-tokio-rustls", "macros"] }
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
`;
  } else if (db === "seaorm-mysql") {
    toml += `sea-orm = { version = "0.12", features = ["sqlx-mysql", "runtime-tokio-rustls", "macros"] }
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
`;
  }

  if (auth) {
    toml += `jsonwebtoken = "9"
axum-extra = { version = "0.9", features = ["typed-header"] }
headers = "0.4"
`;
  }

  return toml;
}
