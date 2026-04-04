use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
{{fields}}
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
