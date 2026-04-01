---
title: Rust Axum API
description: Generate a production-ready Axum web server with async handlers, typed errors, structured logging, and your choice of database.
---

The Rust scaffold generates a production-ready [Axum](https://github.com/tokio-rs/axum) web server with async handlers, typed errors, structured logging, and your chosen database layer.

## Project structure

```
my-app/
├── src/
│   ├── main.rs           # Server setup and startup
│   ├── config.rs         # Environment config (dotenvy + config)
│   ├── errors.rs         # Typed error enum (thiserror)
│   ├── routes/
│   │   ├── mod.rs        # Route registration
│   │   └── health.rs     # GET /health handler
│   └── db/
│       └── mod.rs        # Database pool setup
├── Cargo.toml
├── .env.example
└── README.md
```

With **JWT auth** enabled, you also get:

```
src/
├── auth/
│   ├── mod.rs            # JWT encode/decode helpers
│   ├── middleware.rs     # Axum auth extractor layer
│   └── handlers.rs       # POST /auth/login, POST /auth/register
└── models/
    └── user.rs           # User model + DB queries
```

## Included middleware

| Middleware | Purpose |
|---|---|
| `CorsLayer` | Cross-origin resource sharing (configurable) |
| `TraceLayer` | HTTP request/response tracing |
| `CompressionLayer` | Gzip/brotli response compression |

## Environment variables

The scaffold generates a `.env.example` you copy to `.env`:

```env
DATABASE_URL=postgres://user:password@localhost:5432/mydb
SERVER_HOST=127.0.0.1
SERVER_PORT=3000
# With JWT auth:
JWT_SECRET=change-me-in-production
JWT_EXPIRY_HOURS=24
```

## Adding a route

Create a new handler in `src/routes/`:

```rust
// src/routes/todos.rs
use axum::{extract::State, Json};
use crate::{AppState, errors::AppError};

pub async fn list_todos(
    State(state): State<AppState>,
) -> Result<Json<Vec<Todo>>, AppError> {
    let todos = sqlx::query_as!(Todo, "SELECT * FROM todos")
        .fetch_all(&state.db)
        .await?;
    Ok(Json(todos))
}
```

Register it in `src/routes/mod.rs`:

```rust
Router::new()
    .route("/todos", get(todos::list_todos))
```
