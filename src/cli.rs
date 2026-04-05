use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    name = "create-ruxum-app",
    version,
    about = "Scaffold production-ready Rust Axum APIs and Next.js apps"
)]
pub struct Args {
    /// Project directory name
    pub project_name: Option<String>,

    /// Scaffold type
    #[arg(long, value_enum)]
    pub r#type: Option<ScaffoldTypeArg>,

    // --- Rust Axum options ---
    /// Database backend (Rust)
    #[arg(long, value_enum)]
    pub db: Option<DbArg>,

    /// Add JWT authentication (Rust)
    #[arg(long)]
    pub auth: bool,

    /// Add OpenAPI / Swagger UI documentation (Rust)
    #[arg(long)]
    pub openapi: bool,

    // --- Next.js options ---
    /// Add Tailwind CSS
    #[arg(long)]
    pub tailwind: bool,

    /// Add shadcn/ui (requires --tailwind)
    #[arg(long)]
    pub shadcn: bool,

    /// ORM for Next.js
    #[arg(long, value_enum)]
    pub orm: Option<OrmArg>,

    /// Database provider for ORM
    #[arg(long, value_enum)]
    pub db_provider: Option<DbProviderArg>,

    /// Add NextAuth.js authentication
    #[arg(long)]
    pub next_auth: bool,

    /// Add Jest + React Testing Library
    #[arg(long)]
    pub jest: bool,

    /// Add rspc (type-safe API for Rust + Next.js)
    #[arg(long)]
    pub rspc: bool,

    // --- Install options ---
    /// Run npm install / cargo build after scaffolding
    #[arg(long)]
    pub install: bool,

    /// Skip install step
    #[arg(long, conflicts_with = "install")]
    pub no_install: bool,

    /// Skip confirmation prompt
    #[arg(short, long)]
    pub yes: bool,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum ScaffoldTypeArg {
    Rust,
    Nextjs,
    Fullstack,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum DbArg {
    None,
    SqlxPostgres,
    SqlxMysql,
    SqlxSqlite,
    SeaormPostgres,
    SeaormMysql,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum OrmArg {
    None,
    Prisma,
    Drizzle,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum DbProviderArg {
    Postgres,
    Mysql,
    Sqlite,
}
