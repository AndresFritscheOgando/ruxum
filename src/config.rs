#[derive(Debug, Clone, PartialEq)]
pub enum ScaffoldType {
    Rust,
    Nextjs,
    Fullstack,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DbChoice {
    None,
    SqlxPostgres,
    SqlxMysql,
    SqlxSqlite,
    SeaormPostgres,
    SeaormMysql,
}

impl DbChoice {
    pub fn label(&self) -> &str {
        match self {
            DbChoice::None => "None",
            DbChoice::SqlxPostgres => "SQLx — PostgreSQL",
            DbChoice::SqlxMysql => "SQLx — MySQL",
            DbChoice::SqlxSqlite => "SQLx — SQLite",
            DbChoice::SeaormPostgres => "SeaORM — PostgreSQL",
            DbChoice::SeaormMysql => "SeaORM — MySQL",
        }
    }

    pub fn is_sqlx(&self) -> bool {
        matches!(
            self,
            DbChoice::SqlxPostgres | DbChoice::SqlxMysql | DbChoice::SqlxSqlite
        )
    }

    #[allow(dead_code)]
    pub fn is_seaorm(&self) -> bool {
        matches!(self, DbChoice::SeaormPostgres | DbChoice::SeaormMysql)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Orm {
    None,
    Prisma,
    Drizzle,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DbProvider {
    Postgres,
    Mysql,
    Sqlite,
}

impl DbProvider {
    pub fn label(&self) -> &str {
        match self {
            DbProvider::Postgres => "PostgreSQL",
            DbProvider::Mysql => "MySQL",
            DbProvider::Sqlite => "SQLite",
        }
    }
}

#[derive(Debug, Clone)]
pub struct RustConfig {
    pub db: DbChoice,
    pub auth: bool,
}

#[derive(Debug, Clone)]
pub struct NextjsConfig {
    pub tailwind: bool,
    pub shadcn: bool,
    pub orm: Orm,
    pub db_provider: DbProvider,
    pub next_auth: bool,
    pub jest: bool,
}

#[derive(Debug, Clone)]
pub struct ScaffoldConfig {
    pub project_name: String,
    pub scaffold_type: ScaffoldType,
    pub rust: Option<RustConfig>,
    pub nextjs: Option<NextjsConfig>,
    pub run_install: bool,
}
