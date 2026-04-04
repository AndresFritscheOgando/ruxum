use {{module}}::{{pool_options}};
use crate::config::AppConfig;

pub type Db = sqlx::{{pool_type}};

pub async fn connect(url: &str, config: &AppConfig) -> anyhow::Result<Db> {
    let pool = {{pool_options}}::new()
        .max_connections(config.db_max_connections)
        .min_connections(config.db_min_connections)
        .connect_timeout(config.db_connect_timeout())
        .idle_timeout(config.db_idle_timeout())
        .max_lifetime(config.db_max_lifetime())
        .connect(url)
        .await?;
    Ok(pool)
}
