import { ScaffoldConfig } from "../types";

export function buildDbDieselRs(config: ScaffoldConfig): string {
  const db = config.rust?.db;

  if (db === "diesel-postgres") {
    return `use diesel::PgConnection;
use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, bb8::Pool},
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub type Db = Pool<AsyncPgConnection>;

pub async fn connect(url: &str) -> anyhow::Result<Db> {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);
    let pool = Pool::builder().max_size(10).build(manager).await?;
    Ok(pool)
}

pub async fn run_migrations(database_url: &str) -> anyhow::Result<()> {
    let url = database_url.to_owned();
    tokio::task::spawn_blocking(move || {
        let mut conn = PgConnection::establish(&url)?;
        conn.run_pending_migrations(MIGRATIONS)
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok::<_, anyhow::Error>(())
    })
    .await??;
    Ok(())
}
`;
  }

  if (db === "diesel-mysql") {
    return `use diesel::MysqlConnection;
use diesel_async::{
    AsyncMysqlConnection,
    pooled_connection::{AsyncDieselConnectionManager, bb8::Pool},
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub type Db = Pool<AsyncMysqlConnection>;

pub async fn connect(url: &str) -> anyhow::Result<Db> {
    let manager = AsyncDieselConnectionManager::<AsyncMysqlConnection>::new(url);
    let pool = Pool::builder().max_size(10).build(manager).await?;
    Ok(pool)
}

pub async fn run_migrations(database_url: &str) -> anyhow::Result<()> {
    let url = database_url.to_owned();
    tokio::task::spawn_blocking(move || {
        let mut conn = MysqlConnection::establish(&url)?;
        conn.run_pending_migrations(MIGRATIONS)
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok::<_, anyhow::Error>(())
    })
    .await??;
    Ok(())
}
`;
  }

  // diesel-sqlite
  return `use diesel::SqliteConnection;
use diesel_async::{
    sync_connection_wrapper::SyncConnectionWrapper,
    pooled_connection::{AsyncDieselConnectionManager, bb8::Pool},
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub type Db = Pool<SyncConnectionWrapper<SqliteConnection>>;

pub async fn connect(url: &str) -> anyhow::Result<Db> {
    let manager =
        AsyncDieselConnectionManager::<SyncConnectionWrapper<SqliteConnection>>::new(url);
    let pool = Pool::builder().max_size(10).build(manager).await?;
    Ok(pool)
}

pub async fn run_migrations(database_url: &str) -> anyhow::Result<()> {
    let url = database_url.to_owned();
    tokio::task::spawn_blocking(move || {
        let mut conn = SqliteConnection::establish(&url)?;
        conn.run_pending_migrations(MIGRATIONS)
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok::<_, anyhow::Error>(())
    })
    .await??;
    Ok(())
}
`;
}
