use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::config::{DbProvider, NextjsConfig, Orm};

pub fn scaffold(dir: &Path, cfg: &NextjsConfig, project_name: &str) -> Result<()> {
    let has_orm = cfg.orm != Orm::None;

    fs::create_dir_all(dir.join("src/app"))?;
    fs::create_dir_all(dir.join("src/lib"))?;
    if cfg.shadcn {
        fs::create_dir_all(dir.join("src/components/ui"))?;
    }
    if has_orm {
        match cfg.orm {
            Orm::Prisma => fs::create_dir_all(dir.join("prisma"))?,
            Orm::Drizzle => fs::create_dir_all(dir.join("drizzle"))?,
            Orm::None => {}
        }
    }
    if cfg.next_auth {
        fs::create_dir_all(dir.join("src/app/api/auth/[...nextauth]"))?;
    }
    if cfg.jest {
        fs::create_dir_all(dir.join("__tests__"))?;
    }

    write(dir, "package.json", &package_json(cfg, project_name))?;
    write(dir, "tsconfig.json", tsconfig_json())?;
    write(dir, "next.config.mjs", &next_config(cfg))?;
    write(dir, "src/app/layout.tsx", &layout_tsx(cfg, project_name))?;
    write(dir, "src/app/page.tsx", page_tsx())?;
    write(dir, ".env.example", &env_example(cfg))?;
    write(dir, ".gitignore", nextjs_gitignore())?;
    write(dir, "eslint.config.mjs", eslint_config())?;

    if cfg.tailwind {
        write(dir, "src/app/globals.css", &globals_css(cfg))?;
    } else {
        write(dir, "src/app/globals.css", globals_css_plain())?;
    }

    if cfg.shadcn {
        write(dir, "components.json", &components_json(cfg))?;
    }

    if has_orm {
        write(dir, "src/lib/db.ts", &db_ts(cfg))?;
        match cfg.orm {
            Orm::Prisma => write(dir, "prisma/schema.prisma", &prisma_schema(cfg))?,
            Orm::Drizzle => {
                write(dir, "drizzle/schema.ts", &drizzle_schema(cfg))?;
                write(dir, "drizzle.config.ts", &drizzle_config(cfg))?;
            }
            Orm::None => {}
        }
    }

    if cfg.next_auth {
        write(dir, "src/lib/auth.ts", auth_ts())?;
        write(
            dir,
            "src/app/api/auth/[...nextauth]/route.ts",
            nextauth_route(),
        )?;
    }

    if cfg.jest {
        write(dir, "jest.config.ts", jest_config())?;
        write(dir, "jest.setup.ts", jest_setup())?;
        write(dir, "__tests__/page.test.tsx", page_test())?;
    }

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

fn package_json(cfg: &NextjsConfig, project_name: &str) -> String {
    let template = include_str!("../templates/nextjs/package_json.rs");
    let has_orm = cfg.orm != Orm::None;

    let mut deps: Vec<(&str, &str)> = vec![
        ("next", "16.2.1"),
        ("react", "19.2.4"),
        ("react-dom", "19.2.4"),
    ];

    if cfg.next_auth {
        deps.push(("next-auth", "5.0.0-beta.29"));
    }

    if cfg.orm == Orm::Prisma {
        deps.push(("@prisma/client", "6.9.0"));
    }

    if cfg.orm == Orm::Drizzle {
        match cfg.db_provider {
            DbProvider::Postgres => {
                deps.push(("drizzle-orm", "0.44.0"));
                deps.push(("postgres", "3.4.5"));
            }
            DbProvider::Mysql => {
                deps.push(("drizzle-orm", "0.44.0"));
                deps.push(("mysql2", "3.14.0"));
            }
            DbProvider::Sqlite => {
                deps.push(("drizzle-orm", "0.44.0"));
                deps.push(("better-sqlite3", "11.10.0"));
            }
        }
    }

    let mut dev_deps: Vec<(&str, &str)> = vec![
        ("typescript", "6.0.2"),
        ("@types/node", "25.5.0"),
        ("@types/react", "19.2.14"),
        ("@types/react-dom", "19.2.7"),
        ("eslint", "9.30.0"),
        ("eslint-config-next", "16.2.1"),
    ];

    if cfg.tailwind {
        dev_deps.push(("tailwindcss", "4.2.2"));
        dev_deps.push(("@tailwindcss/postcss", "4.2.2"));
        dev_deps.push(("postcss", "8.5.8"));
        if cfg.shadcn {
            dev_deps.push(("@tailwindcss/typography", "0.5.19"));
        }
    }

    if cfg.orm == Orm::Prisma {
        dev_deps.push(("prisma", "6.9.0"));
    }

    if cfg.orm == Orm::Drizzle {
        dev_deps.push(("drizzle-kit", "0.31.0"));
        if cfg.db_provider == DbProvider::Sqlite {
            dev_deps.push(("@types/better-sqlite3", "7.6.13"));
        }
    }

    if cfg.jest {
        dev_deps.push(("jest", "29.7.0"));
        dev_deps.push(("jest-environment-jsdom", "29.7.0"));
        dev_deps.push(("@testing-library/react", "16.3.0"));
        dev_deps.push(("@testing-library/jest-dom", "6.6.3"));
        dev_deps.push(("@types/jest", "29.5.14"));
        dev_deps.push(("ts-jest", "29.4.0"));
    }

    let deps_str = deps
        .iter()
        .map(|(k, v)| format!("    \"{k}\": \"{v}\""))
        .collect::<Vec<_>>()
        .join(",\n");

    let dev_deps_str = dev_deps
        .iter()
        .map(|(k, v)| format!("    \"{k}\": \"{v}\""))
        .collect::<Vec<_>>()
        .join(",\n");

    let mut scripts = vec![
        ("dev", "next dev"),
        ("build", "next build"),
        ("start", "next start"),
        ("lint", "next lint"),
    ];
    if has_orm && cfg.orm == Orm::Prisma {
        scripts.push(("db:push", "prisma db push"));
        scripts.push(("db:studio", "prisma studio"));
    }
    if has_orm && cfg.orm == Orm::Drizzle {
        scripts.push(("db:push", "drizzle-kit push"));
        scripts.push(("db:studio", "drizzle-kit studio"));
    }
    if cfg.jest {
        scripts.push(("test", "jest"));
        scripts.push(("test:watch", "jest --watch"));
    }

    let scripts_str = scripts
        .iter()
        .map(|(k, v)| format!("    \"{k}\": \"{v}\""))
        .collect::<Vec<_>>()
        .join(",\n");

    template
        .replace("{{project_name}}", project_name)
        .replace("{{scripts}}", &scripts_str)
        .replace("{{dependencies}}", &deps_str)
        .replace("{{devDependencies}}", &dev_deps_str)
}

fn tsconfig_json() -> &'static str {
    include_str!("../templates/nextjs/tsconfig_json.rs")
}

fn next_config(_cfg: &NextjsConfig) -> &'static str {
    include_str!("../templates/nextjs/next_config.rs")
}

fn layout_tsx(cfg: &NextjsConfig, project_name: &str) -> String {
    let template = include_str!("../templates/nextjs/layout_tsx.rs");
    let css_import = if cfg.tailwind {
        "import \"./globals.css\";"
    } else {
        "import \"./globals.css\";"
    };

    template
        .replace("{{project_name}}", project_name)
        .replace("{{css_import}}", css_import)
}

fn page_tsx() -> &'static str {
    include_str!("../templates/nextjs/page_tsx.rs")
}

fn globals_css(cfg: &NextjsConfig) -> String {
    let template = include_str!("../templates/nextjs/globals_css.rs");
    let typography = if cfg.shadcn {
        "\n@plugin \"@tailwindcss/typography\";"
    } else {
        ""
    };

    template.replace("{{typography}}", typography)
}

fn globals_css_plain() -> &'static str {
    "* {\n  box-sizing: border-box;\n  margin: 0;\n  padding: 0;\n}\n"
}

fn components_json(_cfg: &NextjsConfig) -> String {
    r#"{
  "$schema": "https://ui.shadcn.com/schema.json",
  "style": "default",
  "rsc": true,
  "tsx": true,
  "tailwind": {
    "config": "",
    "css": "src/app/globals.css",
    "baseColor": "neutral",
    "cssVariables": true
  },
  "aliases": {
    "components": "@/components",
    "utils": "@/lib/utils",
    "ui": "@/components/ui",
    "lib": "@/lib",
    "hooks": "@/hooks"
  },
  "iconLibrary": "lucide"
}
"#
    .to_string()
}

fn db_ts(cfg: &NextjsConfig) -> String {
    match cfg.orm {
        Orm::Prisma => r#"import { PrismaClient } from "@prisma/client";

const globalForPrisma = globalThis as unknown as { prisma: PrismaClient };

export const db =
  globalForPrisma.prisma ||
  new PrismaClient({
    log: process.env.NODE_ENV === "development" ? ["query"] : [],
  });

if (process.env.NODE_ENV !== "production") globalForPrisma.prisma = db;
"#
        .to_string(),
        Orm::Drizzle => match cfg.db_provider {
            DbProvider::Postgres => r#"import { drizzle } from "drizzle-orm/postgres-js";
import postgres from "postgres";
import * as schema from "../../drizzle/schema";

const client = postgres(process.env.DATABASE_URL!);
export const db = drizzle(client, { schema });
"#
            .to_string(),
            DbProvider::Mysql => r#"import { drizzle } from "drizzle-orm/mysql2";
import mysql from "mysql2/promise";
import * as schema from "../../drizzle/schema";

const client = await mysql.createConnection(process.env.DATABASE_URL!);
export const db = drizzle(client, { schema, mode: "default" });
"#
            .to_string(),
            DbProvider::Sqlite => r#"import { drizzle } from "drizzle-orm/better-sqlite3";
import Database from "better-sqlite3";
import * as schema from "../../drizzle/schema";

const client = new Database(process.env.DATABASE_URL ?? "dev.db");
export const db = drizzle(client, { schema });
"#
            .to_string(),
        },
        Orm::None => String::new(),
    }
}

fn prisma_schema(cfg: &NextjsConfig) -> String {
    let template = include_str!("../templates/nextjs/prisma_schema.rs");
    let provider = match cfg.db_provider {
        DbProvider::Postgres => "postgresql",
        DbProvider::Mysql => "mysql",
        DbProvider::Sqlite => "sqlite",
    };

    template.replace("{{provider}}", provider)
}

fn drizzle_schema(cfg: &NextjsConfig) -> String {
    match cfg.db_provider {
        DbProvider::Postgres => r#"import { pgTable, serial, text, timestamp } from "drizzle-orm/pg-core";

// Add your tables here
// export const users = pgTable("users", {
//   id: serial("id").primaryKey(),
//   email: text("email").notNull().unique(),
//   createdAt: timestamp("created_at").defaultNow().notNull(),
// });
"#
        .to_string(),
        DbProvider::Mysql => r#"import { mysqlTable, serial, text, timestamp } from "drizzle-orm/mysql-core";

// Add your tables here
// export const users = mysqlTable("users", {
//   id: serial("id").primaryKey(),
//   email: text("email").notNull().unique(),
//   createdAt: timestamp("created_at").defaultNow().notNull(),
// });
"#
        .to_string(),
        DbProvider::Sqlite => r#"import { sqliteTable, integer, text } from "drizzle-orm/sqlite-core";

// Add your tables here
// export const users = sqliteTable("users", {
//   id: integer("id").primaryKey({ autoIncrement: true }),
//   email: text("email").notNull().unique(),
// });
"#
        .to_string(),
    }
}

fn drizzle_config(cfg: &NextjsConfig) -> String {
    let template = include_str!("../templates/nextjs/drizzle_config.rs");
    let dialect = match cfg.db_provider {
        DbProvider::Postgres => "postgresql",
        DbProvider::Mysql => "mysql",
        DbProvider::Sqlite => "sqlite",
    };

    template.replace("{{dialect}}", dialect)
}

fn auth_ts() -> &'static str {
    include_str!("../templates/nextjs/auth_config.rs")
}

fn nextauth_route() -> &'static str {
    r#"import { handlers } from "@/lib/auth";

export const { GET, POST } = handlers;
"#
}

fn jest_config() -> &'static str {
    include_str!("../templates/nextjs/jest_config.rs")
}

fn jest_setup() -> &'static str {
    r#"import "@testing-library/jest-dom";
"#
}

fn page_test() -> &'static str {
    r#"import { render, screen } from "@testing-library/react";
import Home from "@/app/page";

describe("Home", () => {
  it("renders the heading", () => {
    render(<Home />);
    expect(screen.getByRole("heading")).toBeInTheDocument();
  });
});
"#
}

fn eslint_config() -> &'static str {
    r#"import { dirname } from "path";
import { fileURLToPath } from "url";
import { FlatCompat } from "@eslint/eslintrc";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const compat = new FlatCompat({ baseDirectory: __dirname });

const eslintConfig = [
  ...compat.extends("next/core-web-vitals", "next/typescript"),
];

export default eslintConfig;
"#
}

fn env_example(cfg: &NextjsConfig) -> String {
    let template = include_str!("../templates/nextjs/env_example.rs");
    let mut db_url = String::new();
    let mut auth_vars = String::new();

    if cfg.orm != Orm::None {
        match cfg.db_provider {
            DbProvider::Postgres => {
                db_url.push_str("DATABASE_URL=postgresql://user:password@localhost:5432/mydb");
            }
            DbProvider::Mysql => {
                db_url.push_str("DATABASE_URL=mysql://user:password@localhost:3306/mydb");
            }
            DbProvider::Sqlite => {
                db_url.push_str("DATABASE_URL=dev.db");
            }
        }
    }

    if cfg.next_auth {
        auth_vars.push_str("AUTH_SECRET=changeme_generate_with_openssl_rand_base64_32\nAUTH_GITHUB_ID=\nAUTH_GITHUB_SECRET=");
    }

    if db_url.is_empty() && auth_vars.is_empty() {
        return "# Add your environment variables here\n".to_string();
    }

    template
        .replace("{{database_url}}", &db_url)
        .replace("{{auth_vars}}", &auth_vars)
}

fn nextjs_gitignore() -> &'static str {
    r#"# Dependencies
node_modules/
.pnp
.pnp.js

# Next.js
.next/
out/

# Production
build/
dist/

# Environment
.env
.env.local
.env.*.local

# Debug
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# Misc
.DS_Store
*.pem
Thumbs.db

# Vercel
.vercel

# TypeScript
*.tsbuildinfo
next-env.d.ts
"#
}
