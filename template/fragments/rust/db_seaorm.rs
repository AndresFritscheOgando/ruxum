use sea_orm::{Database, DatabaseConnection};

pub type Db = DatabaseConnection;

pub async fn connect(url: &str) -> anyhow::Result<Db> {
    let db = Database::connect(url).await?;
    Ok(db)
}
