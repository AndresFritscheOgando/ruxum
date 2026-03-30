use {{module}}::{{pool_options}};

pub type Db = sqlx::{{pool_type}};

pub async fn connect(url: &str) -> anyhow::Result<Db> {
    let pool = {{pool_options}}::new()
        .max_connections(10)
        .connect(url)
        .await?;
    Ok(pool)
}
