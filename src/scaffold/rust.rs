use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::config::{DbChoice, RustConfig};

pub fn scaffold(dir: &Path, cfg: &RustConfig, project_name: &str) -> Result<()> {
    let has_db = cfg.db != DbChoice::None;

    fs::create_dir_all(dir.join("src/handlers"))?;
    if has_db {
        fs::create_dir_all(dir.join("src/models"))?;
    }
    if cfg.auth {
        fs::create_dir_all(dir.join("src/auth"))?;
    }

    write(dir, "Cargo.toml", &cargo_toml(cfg, project_name))?;
    write(dir, "src/main.rs", &main_rs(cfg))?;
    write(dir, "src/router.rs", &router_rs(cfg))?;
    write(dir, "src/config.rs", &config_rs(cfg))?;
    write(dir, "src/errors.rs", errors_rs())?;
    write(dir, "src/handlers/mod.rs", "pub mod health;\n")?;
    write(
        dir,
        "src/handlers/health.rs",
        if cfg.openapi { health_openapi_rs() } else { health_rs() },
    )?;
    if cfg.openapi {
        write(dir, "src/openapi.rs", openapi_rs())?;
    }

    if has_db {
        let db_content = if cfg.db.is_sqlx() {
            db_sqlx_rs(&cfg.db)
        } else {
            db_seaorm_rs().to_string()
        };
        write(dir, "src/db.rs", &db_content)?;
        write(dir, "src/models/mod.rs", "// Add your database models here\n")?;
    }

    if cfg.auth {
        write(dir, "src/auth/mod.rs", "pub mod middleware;\n")?;
        write(dir, "src/auth/middleware.rs", auth_middleware_rs())?;
    }

    write(dir, ".env.example", &env_example(cfg))?;

    Ok(())
}

fn write(dir: &Path, path: &str, content: &str) -> Result<()> {
    let full = dir.join(path);
    if let Some(parent) = full.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(full, content)?;
    Ok(())
}

fn cargo_toml(cfg: &RustConfig, project_name: &str) -> String {
    let template = include_str!("../templates/rust/cargo_toml.rs");
    let mut db_deps = String::new();
    let mut auth_deps = String::new();

    match &cfg.db {
        DbChoice::SqlxPostgres => db_deps.push_str(
            "sqlx = { version = \"0.7\", features = [\"runtime-tokio-rustls\", \"postgres\", \"uuid\", \"chrono\", \"macros\"] }\nuuid = { version = \"1\", features = [\"v4\", \"serde\"] }\nchrono = { version = \"0.4\", features = [\"serde\"] }\n",
        ),
        DbChoice::SqlxMysql => db_deps.push_str(
            "sqlx = { version = \"0.7\", features = [\"runtime-tokio-rustls\", \"mysql\", \"uuid\", \"chrono\", \"macros\"] }\nuuid = { version = \"1\", features = [\"v4\", \"serde\"] }\nchrono = { version = \"0.4\", features = [\"serde\"] }\n",
        ),
        DbChoice::SqlxSqlite => db_deps.push_str(
            "sqlx = { version = \"0.7\", features = [\"runtime-tokio-rustls\", \"sqlite\", \"uuid\", \"chrono\", \"macros\"] }\nuuid = { version = \"1\", features = [\"v4\", \"serde\"] }\nchrono = { version = \"0.4\", features = [\"serde\"] }\n",
        ),
        DbChoice::SeaormPostgres => db_deps.push_str(
            "sea-orm = { version = \"0.12\", features = [\"sqlx-postgres\", \"runtime-tokio-rustls\", \"macros\"] }\nuuid = { version = \"1\", features = [\"v4\", \"serde\"] }\nchrono = { version = \"0.4\", features = [\"serde\"] }\n",
        ),
        DbChoice::SeaormMysql => db_deps.push_str(
            "sea-orm = { version = \"0.12\", features = [\"sqlx-mysql\", \"runtime-tokio-rustls\", \"macros\"] }\nuuid = { version = \"1\", features = [\"v4\", \"serde\"] }\nchrono = { version = \"0.4\", features = [\"serde\"] }\n",
        ),
        DbChoice::None => {}
    }

    if cfg.auth {
        auth_deps.push_str(
            "jsonwebtoken = \"9\"\naxum-extra = { version = \"0.9\", features = [\"typed-header\"] }\nheaders = \"0.4\"\n",
        );
    }

    let openapi_deps = if cfg.openapi {
        "utoipa = { version = \"4\", features = [\"axum_extras\"] }\nutoipa-swagger-ui = { version = \"7\", features = [\"axum\"] }\n"
    } else {
        ""
    };

    template
        .replace("{{project_name}}", project_name)
        .replace("{{db_dependencies}}", &db_deps)
        .replace("{{auth_dependencies}}", &auth_deps)
        .replace("{{openapi_dependencies}}", openapi_deps)
}

fn main_rs(cfg: &RustConfig) -> String {
    let template = include_str!("../templates/rust/main_rs.rs");
    let has_db = cfg.db != DbChoice::None;

    let mods = [
        "mod config;",
        "mod errors;",
        "mod handlers;",
        "mod router;",
        if has_db { "mod db;" } else { "// mod db;" },
        if cfg.auth { "mod auth;" } else { "// mod auth;" },
        if cfg.openapi { "mod openapi;" } else { "// mod openapi;" },
    ]
    .join("\n");

    let state_fields = if has_db {
        "    pub config: config::AppConfig,\n    pub db: db::Db,"
    } else {
        "    pub config: config::AppConfig,"
    };

    let db_connect = if has_db {
        "    let db = db::connect(cfg.database_url.as_deref().unwrap()).await?;\n"
    } else {
        ""
    };

    let state_init = if has_db {
        "    let state = Arc::new(AppState { config: cfg, db });"
    } else {
        "    let state = Arc::new(AppState { config: cfg });"
    };

    let openapi_log = if cfg.openapi {
        "    tracing::info!(\"Swagger UI: http://{}/swagger-ui/\", addr);\n"
    } else {
        ""
    };

    template
        .replace("{{mods}}", &mods)
        .replace("{{state_fields}}", state_fields)
        .replace("{{db_connect}}", db_connect)
        .replace("{{state_init}}", state_init)
        .replace("{{openapi_log}}", openapi_log)
}

fn router_rs(cfg: &RustConfig) -> String {
    let template = include_str!("../templates/rust/router_rs.rs");

    let openapi_imports = if cfg.openapi {
        "use utoipa::OpenApi;\nuse utoipa_swagger_ui::SwaggerUi;\n"
    } else {
        ""
    };

    let openapi_routes = if cfg.openapi {
        "        .merge(SwaggerUi::new(\"/swagger-ui\").url(\"/api-docs/openapi.json\", crate::openapi::ApiDoc::openapi()))\n"
    } else {
        ""
    };

    template
        .replace("{{openapi_imports}}", openapi_imports)
        .replace("{{openapi_routes}}", openapi_routes)
}

fn config_rs(cfg: &RustConfig) -> String {
    let template = include_str!("../templates/rust/config_rs.rs");
    let has_db = cfg.db != DbChoice::None;

    let mut fields = vec![
        "    pub host: String,".to_string(),
        "    pub port: u16,".to_string(),
    ];
    if has_db {
        fields.push("    pub database_url: Option<String>,".to_string());
    }
    if cfg.auth {
        fields.push("    pub jwt_secret: Option<String>,".to_string());
    }
    
    template.replace("{{fields}}", &fields.join("\n"))
}

fn errors_rs() -> &'static str {
    include_str!("../templates/rust/errors_rs.rs")
}

fn health_rs() -> &'static str {
    include_str!("../templates/rust/health_rs.rs")
}

fn db_sqlx_rs(db: &DbChoice) -> String {
    let template = include_str!("../templates/rust/db_sqlx_rs.rs");
    let (pool_options, pool_type, module) = match db {
        DbChoice::SqlxPostgres => ("PgPoolOptions", "PgPool", "sqlx::postgres"),
        DbChoice::SqlxMysql => ("MySqlPoolOptions", "MySqlPool", "sqlx::mysql"),
        DbChoice::SqlxSqlite => ("SqlitePoolOptions", "SqlitePool", "sqlx::sqlite"),
        _ => ("PgPoolOptions", "PgPool", "sqlx::postgres"),
    };

    template
        .replace("{{module}}", module)
        .replace("{{pool_options}}", pool_options)
        .replace("{{pool_type}}", pool_type)
}

fn db_seaorm_rs() -> &'static str {
    include_str!("../templates/rust/db_seaorm_rs.rs")
}

fn auth_middleware_rs() -> &'static str {
    include_str!("../templates/rust/auth_rs.rs")
}

fn health_openapi_rs() -> &'static str {
    include_str!("../templates/rust/health_openapi_rs.rs")
}

fn openapi_rs() -> &'static str {
    include_str!("../templates/rust/openapi_rs.rs")
}

fn env_example(cfg: &RustConfig) -> String {
    let template = include_str!("../templates/rust/env_example.rs");
    let mut db_url = String::new();
    let mut jwt_secret = String::new();

    match &cfg.db {
        DbChoice::SqlxPostgres | DbChoice::SeaormPostgres => {
            db_url.push_str("DATABASE_URL=postgres://user:password@localhost/mydb");
        }
        DbChoice::SqlxMysql | DbChoice::SeaormMysql => {
            db_url.push_str("DATABASE_URL=mysql://user:password@localhost/mydb");
        }
        DbChoice::SqlxSqlite => {
            db_url.push_str("DATABASE_URL=sqlite://./dev.db");
        }
        DbChoice::None => {}
    }

    if cfg.auth {
        jwt_secret.push_str("JWT_SECRET=changeme_use_a_long_random_string_in_production");
    }

    template
        .replace("{{database_url}}", &db_url)
        .replace("{{jwt_secret}}", &jwt_secret)
}
