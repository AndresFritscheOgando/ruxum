use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::config::{DbProvider, NextjsConfig, Orm};

pub fn scaffold(dir: &Path, cfg: &NextjsConfig) -> Result<()> {
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

    write(dir, "package.json", &package_json(cfg))?;
    write(dir, "tsconfig.json", tsconfig_json())?;
    write(dir, "next.config.mjs", &next_config(cfg))?;
    write(dir, "src/app/layout.tsx", &layout_tsx(cfg))?;
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

fn package_json(cfg: &NextjsConfig) -> String {
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

    format!(
        r#"{{
  "name": "my-app",
  "version": "0.1.0",
  "private": true,
  "scripts": {{
{scripts_str}
  }},
  "dependencies": {{
{deps_str}
  }},
  "devDependencies": {{
{dev_deps_str}
  }}
}}
"#
    )
}

fn tsconfig_json() -> &'static str {
    r#"{
  "compilerOptions": {
    "target": "ES2017",
    "lib": ["dom", "dom.iterable", "esnext"],
    "allowJs": true,
    "skipLibCheck": true,
    "strict": true,
    "noEmit": true,
    "esModuleInterop": true,
    "module": "esnext",
    "moduleResolution": "bundler",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "jsx": "preserve",
    "incremental": true,
    "plugins": [{ "name": "next" }],
    "paths": {
      "@/*": ["./src/*"]
    }
  },
  "include": ["next-env.d.ts", "**/*.ts", "**/*.tsx", ".next/types/**/*.ts"],
  "exclude": ["node_modules"]
}
"#
}

fn next_config(_cfg: &NextjsConfig) -> String {
    format!(
        r#"/** @type {{import('next').NextConfig}} */
const nextConfig = {{
  reactStrictMode: true,
}};

export default nextConfig;
"#
    )
}

fn layout_tsx(cfg: &NextjsConfig) -> String {
    let css_import = if cfg.tailwind {
        "import \"./globals.css\";\n"
    } else {
        "import \"./globals.css\";\n"
    };

    format!(
        r#"import type {{ Metadata }} from "next";
{css_import}
export const metadata: Metadata = {{
  title: "My App",
  description: "Generated by create-ruxum-app",
}};

export default function RootLayout({{
  children,
}}: {{
  children: React.ReactNode;
}}) {{
  return (
    <html lang="en">
      <body>{{}}</body>
    </html>
  );
}}
"#,
        // embed children in body
    )
    .replace(
        "<body>{}</body>",
        "<body className=\"antialiased\">{children}</body>",
    )
}

fn page_tsx() -> &'static str {
    r#"export default function Home() {
  return (
    <main>
      <h1>Hello from create-ruxum-app</h1>
    </main>
  );
}
"#
}

fn globals_css(cfg: &NextjsConfig) -> String {
    let typography = if cfg.shadcn {
        "\n@plugin \"@tailwindcss/typography\";"
    } else {
        ""
    };

    format!(
        r#"@import "tailwindcss";{typography}

@theme {{
  --font-sans: var(--font-geist-sans), system-ui, sans-serif;
  --font-mono: var(--font-geist-mono), ui-monospace, monospace;
}}
"#
    )
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
    let provider = match cfg.db_provider {
        DbProvider::Postgres => "postgresql",
        DbProvider::Mysql => "mysql",
        DbProvider::Sqlite => "sqlite",
    };
    let url_env = match cfg.db_provider {
        DbProvider::Sqlite => "env(\"DATABASE_URL\")",
        _ => "env(\"DATABASE_URL\")",
    };

    format!(
        r#"// This is your Prisma schema file,
// learn more about it in the docs: https://pris.ly/d/prisma-schema

generator client {{
  provider = "prisma-client-js"
}}

datasource db {{
  provider = "{provider}"
  url      = {url_env}
}}

// Add your models here
// model User {{
//   id        String   @id @default(cuid())
//   email     String   @unique
//   createdAt DateTime @default(now())
//   updatedAt DateTime @updatedAt
// }}
"#
    )
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
    let dialect = match cfg.db_provider {
        DbProvider::Postgres => "postgresql",
        DbProvider::Mysql => "mysql",
        DbProvider::Sqlite => "sqlite",
    };

    format!(
        r#"import type {{ Config }} from "drizzle-kit";

export default {{
  schema: "./drizzle/schema.ts",
  out: "./drizzle/migrations",
  dialect: "{dialect}",
  dbCredentials: {{
    url: process.env.DATABASE_URL!,
  }},
}} satisfies Config;
"#
    )
}

fn auth_ts() -> &'static str {
    r#"import NextAuth from "next-auth";
import GitHub from "next-auth/providers/github";

export const { handlers, signIn, signOut, auth } = NextAuth({
  providers: [
    GitHub({
      clientId: process.env.AUTH_GITHUB_ID!,
      clientSecret: process.env.AUTH_GITHUB_SECRET!,
    }),
  ],
  callbacks: {
    authorized({ auth }) {
      return !!auth?.user;
    },
  },
});
"#
}

fn nextauth_route() -> &'static str {
    r#"import { handlers } from "@/lib/auth";

export const { GET, POST } = handlers;
"#
}

fn jest_config() -> &'static str {
    r#"import type { Config } from "jest";
import nextJest from "next/jest.js";

const createJestConfig = nextJest({ dir: "./" });

const config: Config = {
  coverageProvider: "v8",
  testEnvironment: "jsdom",
  setupFilesAfterFramework: ["<rootDir>/jest.setup.ts"],
};

export default createJestConfig(config);
"#
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
    let mut s = String::new();

    if cfg.orm != Orm::None {
        match cfg.db_provider {
            DbProvider::Postgres => {
                s.push_str("DATABASE_URL=postgresql://user:password@localhost:5432/mydb\n");
            }
            DbProvider::Mysql => {
                s.push_str("DATABASE_URL=mysql://user:password@localhost:3306/mydb\n");
            }
            DbProvider::Sqlite => {
                s.push_str("DATABASE_URL=dev.db\n");
            }
        }
    }

    if cfg.next_auth {
        s.push_str("AUTH_SECRET=changeme_generate_with_openssl_rand_base64_32\n");
        s.push_str("AUTH_GITHUB_ID=\n");
        s.push_str("AUTH_GITHUB_SECRET=\n");
    }

    if s.is_empty() {
        s.push_str("# Add your environment variables here\n");
    }

    s
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
