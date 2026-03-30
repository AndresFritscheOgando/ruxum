import { ScaffoldConfig } from "../types";

export function buildConfigRs(config: ScaffoldConfig): string {
  const db = config.rust?.db;
  const auth = config.rust?.auth;
  const hasDb = db && db !== "none";

  const fields = [
    "    pub host: String,",
    "    pub port: u16,",
    hasDb ? "    pub database_url: Option<String>," : null,
    auth ? "    pub jwt_secret: Option<String>," : null,
  ]
    .filter(Boolean)
    .join("\n");

  return `use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
${fields}
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let cfg = config::Config::builder()
            .add_source(config::Environment::default())
            .set_default("host", "127.0.0.1")?
            .set_default("port", 3000)?
            .build()?;
        Ok(cfg.try_deserialize()?)
    }
}
`;
}
