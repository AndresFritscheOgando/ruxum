# create-ruxum-app

[![npm version](https://img.shields.io/npm/v/create-ruxum-app?color=cyan)](https://www.npmjs.com/package/create-ruxum-app)
[![Node.js](https://img.shields.io/node/v/create-ruxum-app)](https://nodejs.org)

Scaffold a production-ready **Rust + Axum** REST API in seconds — no boilerplate, no guesswork.

```sh
npx create-ruxum-app@latest
```

---

## Overview

`create-ruxum-app` is an interactive CLI that generates a fully wired Axum project tailored to your choices. Pick a database layer, optionally add JWT authentication, and get a compilable Rust codebase with sensible defaults already in place:

- Structured logging via `tracing` + `tracing-subscriber`
- Configuration loading via `dotenvy` + `config`
- CORS and HTTP trace middleware via `tower-http`
- Typed error handling with `thiserror` + `anyhow`
- A `/health` endpoint out of the box

---

## Prerequisites

| Requirement | Minimum version |
|---|---|
| Node.js | 18.0.0 |
| Rust + Cargo | stable (latest recommended) |

If you do not have Rust installed, get it from [rustup.rs](https://rustup.rs).

---

## Quick Start

```sh
npx create-ruxum-app@latest
cd my-axum-app
cp .env.example .env
cargo run
```

Your API will be listening at `http://127.0.0.1:3000`.

Verify it is running:

```sh
curl http://127.0.0.1:3000/health
# {"status":"ok","version":"0.1.0"}
```

---

## Interactive Wizard

Running the CLI launches a step-by-step wizard:

```
  ██████╗ ██╗   ██╗██╗  ██╗██╗   ██╗███╗   ███╗
  ██╔══██╗██║   ██║╚██╗██╔╝██║   ██║████╗ ████║
  ██████╔╝██║   ██║ ╚███╔╝ ██║   ██║██╔████╔██║
  ██╔══██╗██║   ██║ ██╔██╗ ██║   ██║██║╚██╔╝██║
  ██║  ██║╚██████╔╝██╔╝ ██╗╚██████╔╝██║ ╚═╝ ██║
  ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚═╝     ╚═╝

  Scaffold a production-ready Rust Axum API

◆  What is your project named?
◆  Which database would you like to use?
◆  Add JWT authentication?
◆  Scaffold this project?
```

### Prompt reference

| Prompt | Description |
|---|---|
| **Project name** | The Rust crate name. Must contain only letters, numbers, hyphens (`-`), and underscores (`_`). Defaults to `my-axum-app`. |
| **Database** | The database layer to include. See [Database Options](#database-options). |
| **JWT authentication** | Adds a JWT Bearer extractor and helper functions. See [JWT Authentication](#jwt-authentication). |
| **Confirm** | Displays a summary and asks for final confirmation before writing any files. If the target directory already exists, you will be asked whether to overwrite it. |

---

## Database Options

| Option | Crate | Engine |
|---|---|---|
| None | — | No database wired |
| SQLx — PostgreSQL | `sqlx` 0.7 | PostgreSQL |
| SQLx — MySQL | `sqlx` 0.7 | MySQL |
| SQLx — SQLite | `sqlx` 0.7 | SQLite (file-based) |
| SeaORM — PostgreSQL | `sea-orm` 0.12 | PostgreSQL |
| SeaORM — MySQL | `sea-orm` 0.12 | MySQL |

When a database is selected, `uuid` and `chrono` are added automatically. A `src/db.rs` connection module and a `src/models/mod.rs` placeholder are generated alongside the rest of the project.

---

## What Gets Scaffolded

### Base project (all configurations)

```
my-axum-app/
├── Cargo.toml
├── .env.example
└── src/
    ├── main.rs          # Tokio entry point, AppState, server bootstrap
    ├── router.rs        # Axum router with CORS + trace middleware
    ├── config.rs        # AppConfig loaded from environment
    ├── errors.rs        # AppError enum (thiserror) with IntoResponse impl
    └── handlers/
        ├── mod.rs
        └── health.rs    # GET /health → {"status":"ok","version":"..."}
```

### With a database

```
└── src/
    ├── db.rs            # Connection pool initialisation
    └── models/
        └── mod.rs       # Model definitions placeholder
```

### With JWT authentication

```
└── src/
    └── auth/
        ├── mod.rs
        └── middleware.rs  # Claims, encode_token, decode_token, AuthUser extractor
```

---

## Environment Variables

The generated `.env.example` contains the variables your project needs. Copy it to `.env` before running.

| Variable | Present when | Description |
|---|---|---|
| `HOST` | Always | Bind address. Defaults to `127.0.0.1`. |
| `PORT` | Always | Bind port. Defaults to `3000`. |
| `DATABASE_URL` | Database selected | Connection string for your database. |
| `JWT_SECRET` | Auth enabled | Secret key used to sign and verify JWTs. **Use a long, random string in production.** |

### Example values

```env
HOST=127.0.0.1
PORT=3000

# PostgreSQL
DATABASE_URL=postgres://user:password@localhost/mydb

# MySQL
# DATABASE_URL=mysql://user:password@localhost/mydb

# SQLite
# DATABASE_URL=sqlite://./dev.db

# Auth
JWT_SECRET=changeme_use_a_long_random_string_in_production
```

---

## JWT Authentication

When JWT auth is enabled, the scaffolder adds `src/auth/middleware.rs` with the following ready-to-use utilities:

| Item | Description |
|---|---|
| `Claims` | Serialisable struct with `sub` (subject) and `exp` (expiry) fields. |
| `encode_token(secret, sub, exp)` | Creates a signed JWT string. |
| `decode_token(secret, token)` | Validates and decodes a JWT string, returning `Claims`. |
| `AuthUser` | An Axum `FromRequestParts` extractor that reads the `Authorization: Bearer <token>` header and rejects unauthenticated requests with a typed `AppError::Unauthorized`. |

To protect a route, add `AuthUser` as an extractor parameter in your handler:

```rust
pub async fn me(auth: AuthUser) -> Json<Value> {
    Json(json!({ "sub": auth.0.sub }))
}
```

---

## Contributing

Issues and pull requests are welcome at [github.com/AndresFritscheOgando/ruxum](https://github.com/AndresFritscheOgando/ruxum).
