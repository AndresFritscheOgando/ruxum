use anyhow::Result;
use console::Style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

use crate::cli::{Args, DbArg, DbProviderArg, OrmArg, ScaffoldTypeArg};
use crate::config::{
    DbChoice, DbProvider, NextjsConfig, Orm, RustConfig, ScaffoldConfig, ScaffoldType,
};

pub fn resolve(args: Args) -> Result<ScaffoldConfig> {
    let theme = ColorfulTheme::default();
    let bold = Style::new().bold();

    println!();
    println!(
        "{}",
        bold.apply_to("  create-ruxum-app — scaffold your next project")
    );
    println!();

    // --- Scaffold type ---
    let scaffold_type = match args.r#type {
        Some(ScaffoldTypeArg::Rust) => ScaffoldType::Rust,
        Some(ScaffoldTypeArg::Nextjs) => ScaffoldType::Nextjs,
        Some(ScaffoldTypeArg::Fullstack) => ScaffoldType::Fullstack,
        None => {
            let options = &["Rust Axum API", "Next.js App", "Full-stack (Axum + Next.js)"];
            let idx = Select::with_theme(&theme)
                .with_prompt("What would you like to scaffold?")
                .items(options)
                .default(0)
                .interact()?;
            match idx {
                0 => ScaffoldType::Rust,
                1 => ScaffoldType::Nextjs,
                _ => ScaffoldType::Fullstack,
            }
        }
    };

    // --- Project name ---
    let project_name = match args.project_name {
        Some(n) => n,
        None => {
            let default = "my-app".to_string();
            Input::with_theme(&theme)
                .with_prompt("Project name")
                .default(default)
                .validate_with(|input: &String| {
                    if input
                        .chars()
                        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
                    {
                        Ok(())
                    } else {
                        Err("Only letters, numbers, hyphens, and underscores allowed")
                    }
                })
                .interact_text()?
        }
    };

    // --- Rust options ---
    let rust = if matches!(scaffold_type, ScaffoldType::Rust | ScaffoldType::Fullstack) {
        let db = match args.db {
            Some(DbArg::None) => DbChoice::None,
            Some(DbArg::SqlxPostgres) => DbChoice::SqlxPostgres,
            Some(DbArg::SqlxMysql) => DbChoice::SqlxMysql,
            Some(DbArg::SqlxSqlite) => DbChoice::SqlxSqlite,
            Some(DbArg::SeaormPostgres) => DbChoice::SeaormPostgres,
            Some(DbArg::SeaormMysql) => DbChoice::SeaormMysql,
            None => {
                let options = &[
                    "None",
                    "SQLx — PostgreSQL",
                    "SQLx — MySQL",
                    "SQLx — SQLite",
                    "SeaORM — PostgreSQL",
                    "SeaORM — MySQL",
                ];
                let idx = Select::with_theme(&theme)
                    .with_prompt("Database (Rust)")
                    .items(options)
                    .default(0)
                    .interact()?;
                match idx {
                    0 => DbChoice::None,
                    1 => DbChoice::SqlxPostgres,
                    2 => DbChoice::SqlxMysql,
                    3 => DbChoice::SqlxSqlite,
                    4 => DbChoice::SeaormPostgres,
                    _ => DbChoice::SeaormMysql,
                }
            }
        };

        let auth = if args.auth {
            true
        } else {
            Confirm::with_theme(&theme)
                .with_prompt("Add JWT authentication?")
                .default(false)
                .interact()?
        };

        let openapi = if args.openapi {
            true
        } else {
            Confirm::with_theme(&theme)
                .with_prompt("Add OpenAPI / Swagger UI documentation?")
                .default(false)
                .interact()?
        };

        Some(RustConfig { db, auth, openapi })
    } else {
        None
    };

    // --- Next.js options ---
    let nextjs = if matches!(scaffold_type, ScaffoldType::Nextjs | ScaffoldType::Fullstack) {
        let tailwind = if args.tailwind {
            true
        } else {
            Confirm::with_theme(&theme)
                .with_prompt("Add Tailwind CSS?")
                .default(true)
                .interact()?
        };

        let shadcn = if !tailwind {
            false
        } else if args.shadcn {
            true
        } else {
            Confirm::with_theme(&theme)
                .with_prompt("Add shadcn/ui?")
                .default(true)
                .interact()?
        };

        let orm = match args.orm {
            Some(OrmArg::None) => Orm::None,
            Some(OrmArg::Prisma) => Orm::Prisma,
            Some(OrmArg::Drizzle) => Orm::Drizzle,
            None => {
                let options = &["None", "Prisma", "Drizzle"];
                let idx = Select::with_theme(&theme)
                    .with_prompt("Database ORM (Next.js)")
                    .items(options)
                    .default(0)
                    .interact()?;
                match idx {
                    0 => Orm::None,
                    1 => Orm::Prisma,
                    _ => Orm::Drizzle,
                }
            }
        };

        let db_provider = if orm == Orm::None {
            DbProvider::Postgres
        } else {
            match args.db_provider {
                Some(DbProviderArg::Postgres) => DbProvider::Postgres,
                Some(DbProviderArg::Mysql) => DbProvider::Mysql,
                Some(DbProviderArg::Sqlite) => DbProvider::Sqlite,
                None => {
                    let options = &["PostgreSQL", "MySQL", "SQLite"];
                    let idx = Select::with_theme(&theme)
                        .with_prompt("Database provider")
                        .items(options)
                        .default(0)
                        .interact()?;
                    match idx {
                        0 => DbProvider::Postgres,
                        1 => DbProvider::Mysql,
                        _ => DbProvider::Sqlite,
                    }
                }
            }
        };

        let next_auth = if args.next_auth {
            true
        } else {
            Confirm::with_theme(&theme)
                .with_prompt("Add NextAuth.js?")
                .default(false)
                .interact()?
        };

        let jest = if args.jest {
            true
        } else {
            Confirm::with_theme(&theme)
                .with_prompt("Add Jest + React Testing Library?")
                .default(false)
                .interact()?
        };

        let rspc = if args.rspc {
            true
        } else {
            Confirm::with_theme(&theme)
                .with_prompt("Add rspc (type-safe API for Rust + Next.js)?")
                .default(true)
                .interact()?
        };

        Some(NextjsConfig {
            tailwind,
            shadcn,
            orm,
            db_provider,
            next_auth,
            jest,
            rspc,
        })
    } else {
        None
    };

    // --- Confirmation summary ---
    if !args.yes {
        println!();
        print_summary(&project_name, &scaffold_type, &rust, &nextjs);
        let confirmed = Confirm::with_theme(&theme)
            .with_prompt("Scaffold this project?")
            .default(true)
            .interact()?;
        if !confirmed {
            println!("Cancelled.");
            std::process::exit(0);
        }
    }

    // --- Install step ---
    let run_install = if args.no_install {
        false
    } else if args.install {
        true
    } else {
        let needs_npm = matches!(scaffold_type, ScaffoldType::Nextjs | ScaffoldType::Fullstack);
        let needs_cargo = matches!(scaffold_type, ScaffoldType::Rust | ScaffoldType::Fullstack);
        let prompt = match (needs_npm, needs_cargo) {
            (true, true) => "Run npm install + cargo build?",
            (true, false) => "Run npm install?",
            (false, true) => "Run cargo build?",
            _ => unreachable!(),
        };
        Confirm::with_theme(&theme)
            .with_prompt(prompt)
            .default(true)
            .interact()?
    };

    println!();

    Ok(ScaffoldConfig {
        project_name,
        scaffold_type,
        rust,
        nextjs,
        run_install,
    })
}

fn print_summary(
    name: &str,
    scaffold_type: &ScaffoldType,
    rust: &Option<RustConfig>,
    nextjs: &Option<NextjsConfig>,
) {
    let dim = Style::new().dim();
    let bold = Style::new().bold();
    let w = 35usize;

    let divider = dim.apply_to(format!("  ┌{}┐", "─".repeat(w)));
    let bottom = dim.apply_to(format!("  └{}┘", "─".repeat(w)));

    let row = |label: &str, value: &str| {
        let line = format!("  │  {:<12}{}", label, bold.apply_to(value));
        let pad = w + 4 - label.len() - value.len() - 2;
        format!("{}{}│", line, " ".repeat(pad.max(1)))
    };

    let type_label = match scaffold_type {
        ScaffoldType::Rust => "Rust Axum",
        ScaffoldType::Nextjs => "Next.js",
        ScaffoldType::Fullstack => "Full-stack",
    };

    println!("{}", divider);
    println!("{}", row("Project:", name));
    println!("{}", row("Type:", type_label));

    if let Some(r) = rust {
        println!("{}", row("Rust DB:", r.db.label()));
        println!("{}", row("JWT Auth:", if r.auth { "Yes" } else { "No" }));
        println!("{}", row("OpenAPI:", if r.openapi { "Yes" } else { "No" }));
    }

    if let Some(n) = nextjs {
        let orm_label = match &n.orm {
            Orm::None => "None".to_string(),
            Orm::Prisma => format!("Prisma ({})", n.db_provider.label()),
            Orm::Drizzle => format!("Drizzle ({})", n.db_provider.label()),
        };
        println!("{}", row("Tailwind:", if n.tailwind { "Yes" } else { "No" }));
        println!("{}", row("shadcn/ui:", if n.shadcn { "Yes" } else { "No" }));
        println!("{}", row("ORM:", &orm_label));
        println!(
            "{}",
            row("NextAuth:", if n.next_auth { "Yes" } else { "No" })
        );
        println!("{}", row("Jest:", if n.jest { "Yes" } else { "No" }));
        println!("{}", row("rspc API:", if n.rspc { "Yes" } else { "No" }));
    }

    println!("{}", bottom);
    println!();
}
