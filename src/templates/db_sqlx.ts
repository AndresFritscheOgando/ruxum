import { ScaffoldConfig, DbChoice } from "../types";

function sqlxTypes(db: DbChoice): { poolOptions: string; poolType: string; module: string } {
  switch (db) {
    case "sqlx-postgres":
      return { poolOptions: "PgPoolOptions", poolType: "PgPool", module: "sqlx::postgres" };
    case "sqlx-mysql":
      return { poolOptions: "MySqlPoolOptions", poolType: "MySqlPool", module: "sqlx::mysql" };
    case "sqlx-sqlite":
      return { poolOptions: "SqlitePoolOptions", poolType: "SqlitePool", module: "sqlx::sqlite" };
    default:
      return { poolOptions: "PgPoolOptions", poolType: "PgPool", module: "sqlx::postgres" };
  }
}

export function buildDbSqlxRs(config: ScaffoldConfig): string {
  const { poolOptions, poolType, module } = sqlxTypes(config.db);
  return `use ${module}::${poolOptions};

pub type Db = sqlx::${poolType};

pub async fn connect(url: &str) -> anyhow::Result<Db> {
    let pool = ${poolOptions}::new()
        .max_connections(10)
        .connect(url)
        .await?;
    Ok(pool)
}
`;
}
