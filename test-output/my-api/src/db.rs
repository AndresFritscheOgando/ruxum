use sqlx::postgres::PgPoolOptions;

pub type Db = sqlx::PgPool;

pub async fn connect(url: &str) -> anyhow::Result<Db> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(url)
        .await?;
    Ok(pool)
}
