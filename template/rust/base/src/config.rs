use anyhow::Result;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
{{fields}}
    #[serde(default = "default_db_max_connections")]
    pub db_max_connections: u32,
    #[serde(default = "default_db_min_connections")]
    pub db_min_connections: u32,
    #[serde(default = "default_db_connect_timeout_secs")]
    pub db_connect_timeout_secs: u64,
    #[serde(default = "default_db_idle_timeout_secs")]
    pub db_idle_timeout_secs: u64,
    #[serde(default = "default_db_max_lifetime_secs")]
    pub db_max_lifetime_secs: u64,
}

fn default_db_max_connections() -> u32 {
    5
}

fn default_db_min_connections() -> u32 {
    1
}

fn default_db_connect_timeout_secs() -> u64 {
    30
}

fn default_db_idle_timeout_secs() -> u64 {
    600 // 10 minutes
}

fn default_db_max_lifetime_secs() -> u64 {
    1800 // 30 minutes
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let cfg = config::Config::builder()
            .add_source(config::Environment::default())
            .set_default("host", "127.0.0.1")?
            .set_default("port", 3000)?
            .set_default("db_max_connections", default_db_max_connections())?
            .set_default("db_min_connections", default_db_min_connections())?
            .set_default("db_connect_timeout_secs", default_db_connect_timeout_secs())?
            .set_default("db_idle_timeout_secs", default_db_idle_timeout_secs())?
            .set_default("db_max_lifetime_secs", default_db_max_lifetime_secs())?
            .build()?;
        let config: Self = cfg.try_deserialize()?;
        config.validate()?;
        Ok(config)
    }

    /// Validate that all required secrets and configuration are present
    fn validate(&self) -> Result<()> {
        // Validate JWT secret is present if auth is used
        if self.jwt_secret.as_ref().map_or(true, |s| s.is_empty()) {
            return Err(anyhow::anyhow!(
                "JWT_SECRET environment variable is required but not set or is empty. \
                 This is required for authentication. Set it to a strong, random value."
            ));
        }

        // Validate database settings if database_url is present
        if let Some(db_url) = &self.database_url {
            if db_url.is_empty() {
                return Err(anyhow::anyhow!(
                    "DATABASE_URL is set but empty. Provide a valid database connection string."
                ));
            }
        }

        Ok(())
    }

    pub fn db_connect_timeout(&self) -> Duration {
        Duration::from_secs(self.db_connect_timeout_secs)
    }

    pub fn db_idle_timeout(&self) -> Option<Duration> {
        if self.db_idle_timeout_secs > 0 {
            Some(Duration::from_secs(self.db_idle_timeout_secs))
        } else {
            None
        }
    }

    pub fn db_max_lifetime(&self) -> Option<Duration> {
        if self.db_max_lifetime_secs > 0 {
            Some(Duration::from_secs(self.db_max_lifetime_secs))
        } else {
            None
        }
    }
}
