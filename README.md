# Ruxum

[![npm version](https://img.shields.io/npm/v/create-ruxum-app?color=cyan)](https://www.npmjs.com/package/create-ruxum-app)
[![Node.js](https://img.shields.io/node/v/create-ruxum-app)](https://nodejs.org)

Scaffold a production-ready **Rust + Axum API** and **Next.js app** in seconds — no boilerplate, no guesswork.

```sh
npx create-ruxum-app@latest
```

---

## Overview

`create-ruxum-app` is an interactive CLI that generates fully wired Rust Axum and Next.js projects tailored to your choices. Create standalone APIs, web applications, or complete full-stack projects with zero configuration.

### What You Get

**Rust Axum Backend:**
- Structured logging via `tracing` + `tracing-subscriber`
- Configuration loading via `dotenvy` + `config`
- CORS and HTTP trace middleware via `tower-http`
- Typed error handling with `thiserror` + `anyhow`
- Database support (SQLx, SeaORM) with migrations
- Optional JWT authentication
- A `/health` endpoint out of the box

**Next.js Frontend:**
- TypeScript with strict mode enabled
- Environment variable validation (Zod schema)
- Optional Next-Auth integration
- Optional Tailwind CSS + shadcn/ui
- Optional database ORM (Prisma, Drizzle)
- Jest testing setup
- ESLint + TypeScript linting

**Full-Stack Project:**
- Monorepo structure with `/api` and `/www`
- Both Rust API and Next.js web in one command
- Shared type systems via TypeScript (frontend-to-backend)
- Ready for deployment

---

## Prerequisites

| Requirement | Minimum version |
|---|---|
| Node.js | 18.0.0 |
| Rust + Cargo | stable (latest recommended) |

If you do not have Rust installed, get it from [rustup.rs](https://rustup.rs).

---

## Quick Start

### Rust Axum API

```sh
npx create-ruxum-app@latest --type rust
cd my-api
cp .env.example .env
cargo run
```

Your API will be listening at `http://127.0.0.1:3000`.

Verify it is running:

```sh
curl http://127.0.0.1:3000/health
# {"status":"ok","version":"0.1.0"}
```

### Next.js Application

```sh
npx create-ruxum-app@latest --type nextjs
cd my-web
cp .env.example .env.local
npm install
npm run dev
```

Your app will be at `http://localhost:3000`.

### Full-Stack Project

```sh
npx create-ruxum-app@latest --type fullstack
cd my-project
# Configure and run both API and frontend
cd api && cp .env.example .env && cargo run
# In another terminal:
cd www && cp .env.example .env.local && npm install && npm run dev
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

  Scaffold production-ready Rust & Next.js projects

◆  What project type?
   ◇ Rust Axum API
   ◇ Next.js Application
   ◇ Full-Stack (Rust + Next.js)
◆  What is your project named?
◆  [Additional prompts based on selection]
```

### Prompt Reference

| Prompt | Description |
|---|---|
| **Project type** | Choose between Rust API, Next.js, or Full-Stack. |
| **Project name** | Directory name for the project. Must be valid for both Node.js and Rust. |
| **Database** (Rust) | SQLx or SeaORM with PostgreSQL, MySQL, or SQLite. See [Database Options](#database-options). |
| **Auth** (Rust) | Adds JWT authentication middleware. |
| **ORM** (Next.js) | Optional: Prisma or Drizzle for database access. |
| **Next-Auth** (Next.js) | Optional: GitHub OAuth + session management. |
| **Styling** (Next.js) | Optional: Tailwind CSS + shadcn/ui components. |
| **Testing** (Next.js) | Optional: Jest configuration with React Testing Library. |
| **Confirm** | Review your selections and create the project. |

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

### Rust Axum API

#### Base project (all configurations)

```
my-api/
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

#### With a database

```
└── src/
    ├── db.rs            # Connection pool initialisation
    └── models/
        └── mod.rs       # Model definitions placeholder
```

#### With JWT authentication

```
└── src/
    └── auth/
        ├── mod.rs
        └── middleware.rs  # Claims, encode_token, decode_token, AuthUser extractor
```

### Next.js Application

#### Base project (all configurations)

```
my-web/
├── package.json
├── tsconfig.json
├── next.config.mjs
├── .env.example
├── .eslintrc.mjs
└── src/
    ├── app/
    │   ├── layout.tsx      # Root layout with metadata
    │   ├── page.tsx        # Home page
    │   └── globals.css     # Global styles
    └── lib/
        ├── env.ts         # Environment variable validation (Zod)
```

#### With ORM (Prisma or Drizzle)

```
├── prisma/
│   ├── schema.prisma    # OR
├── drizzle/
│   └── schema.ts        # Database schema
└── src/lib/
    └── db.ts            # Database connection & client
```

#### With Next-Auth

```
└── src/
    ├── lib/
    │   └── auth.ts      # NextAuth configuration
    └── app/api/auth/[...nextauth]/
        └── route.ts     # Auth API route
```

#### With Tailwind + shadcn/ui

```
└── src/
    ├── components/
    │   └── ui/          # shadcn/ui components
    └── lib/
        └── utils.ts     # cn() helper for Tailwind merging
```

#### With Jest Testing

```
├── jest.config.ts
├── jest.setup.ts
└── __tests__/
    └── page.test.tsx    # Example test
```

### Full-Stack Project

Combines both in a monorepo structure:

```
my-project/
├── README.md            # Getting started guide
├── api/                 # Rust Axum backend
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── www/                 # Next.js frontend
    ├── package.json
    └── src/
        └── app/
            └── page.tsx
```

---

## Template Validation

Both Rust and Next.js templates are validated after scaffolding:

- **Rust**: Runs `cargo check` to verify syntax
- **Next.js**: Runs `tsc --noEmit` to check TypeScript

Validation happens automatically and shows:
- ✓ Success message if templates are correct
- ⚠ Warning if validation tools aren't installed (continues anyway)

This ensures you get working code from day one. See [TEMPLATE_VALIDATION.md](docs/TEMPLATE_VALIDATION.md) for details.

---

## Environment Variables

All generated projects include `.env.example` with the variables your project needs.

### Rust Axum

| Variable | Present when | Description |
|---|---|---|
| `HOST` | Always | Bind address. Defaults to `127.0.0.1`. |
| `PORT` | Always | Bind port. Defaults to `3000`. |
| `DATABASE_URL` | Database selected | Connection string for your database. |
| `JWT_SECRET` | Auth enabled | Secret key used to sign and verify JWTs. |

### Next.js

| Variable | Present when | Description |
|---|---|---|
| `DATABASE_URL` | Database selected | Connection string for Prisma or Drizzle. |
| `AUTH_GITHUB_ID` | Next-Auth enabled | GitHub OAuth app ID. |
| `AUTH_GITHUB_SECRET` | Next-Auth enabled | GitHub OAuth app secret. |
| `AUTH_SECRET` | Next-Auth enabled | Random key for session encryption. |

All Next.js environment variables are **validated at startup** using Zod. Missing required variables will show clear error messages.

### Example values (Rust)

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

### Example values (Next.js)

```env
# GitHub OAuth (get from https://github.com/settings/developers)
AUTH_GITHUB_ID=your_github_oauth_app_id
AUTH_GITHUB_SECRET=your_github_oauth_app_secret

# Generate with: openssl rand -base64 32
AUTH_SECRET=your_random_secret_key_for_auth

# Database (optional, only if ORM selected)
DATABASE_URL=postgresql://user:password@localhost:5432/dbname
```

---

## Authentication

### Rust Axum — JWT

When JWT auth is enabled, the scaffolder adds `src/auth/middleware.rs` with ready-to-use utilities:

| Item | Description |
|---|---|
| `Claims` | Serializable struct with `sub` (subject) and `exp` (expiry) fields. |
| `encode_token(secret, sub, exp)` | Creates a signed JWT string. |
| `decode_token(secret, token)` | Validates and decodes a JWT string, returning `Claims`. |
| `AuthUser` | An Axum `FromRequestParts` extractor that reads the `Authorization: Bearer <token>` header and rejects unauthenticated requests with a typed `AppError::Unauthorized`. |

To protect a route, add `AuthUser` as an extractor parameter:

```rust
pub async fn me(auth: AuthUser) -> Json<Value> {
    Json(json!({ "sub": auth.0.sub }))
}
```

### Next.js — NextAuth

When Next-Auth is enabled, the scaffolder generates:

| Item | Description |
|---|---|
| `src/lib/auth.ts` | NextAuth configuration with GitHub OAuth provider |
| `src/app/api/auth/[...nextauth]/route.ts` | Auth API route |
| Environment validation | Zod schema ensures `AUTH_SECRET`, `AUTH_GITHUB_ID`, `AUTH_GITHUB_SECRET` are present at startup |

To protect a route or component:

```typescript
import { auth } from "@/lib/auth";

export default async function ProtectedPage() {
  const session = await auth();
  
  if (!session) {
    redirect("/api/auth/signin");
  }
  
  return <h1>Welcome, {session.user?.name}</h1>;
}
```

---

## Key Features

✨ **Zero Configuration**
- Sensible defaults for all frameworks
- Pre-configured tooling (routing, auth, ORM, testing)

🚀 **Production-Ready**
- TypeScript by default
- Error handling and validation built-in
- Environment variable validation (no runtime surprises)
- Testing setup included

🔒 **Secure by Default**
- No non-null assertions in generated code
- Validated environment variables with Zod
- CORS middleware configured
- JWT auth available for APIs
- OAuth integration available for web apps

✅ **Syntax Validation**
- `cargo check` runs automatically after Rust scaffolding
- `tsc --noEmit` runs automatically after Next.js scaffolding
- Catch errors before you run the code

🛠️ **Full-Stack Ready**
- Scaffold both API and frontend in one command
- Monorepo structure for easy management
- Shared types between frontend and backend

---

## Documentation

### For Users

- [Template Validation](docs/TEMPLATE_VALIDATION.md) — How syntax validation works
- [Environment Validation Guide](template/fragments/nextjs/ENV_VALIDATION_GUIDE.md) — Safe environment variables

### For Template Developers

- [Next.js Template Integration](template/fragments/nextjs/TEMPLATE_INTEGRATION.md) — How to add new Next.js templates
- [Rust Template Integration](template/fragments/rust/TEMPLATE_INTEGRATION.md) — How to add new Rust templates

---

## Contributing

Issues and pull requests are welcome at [github.com/AndresFritscheOgando/ruxum](https://github.com/AndresFritscheOgando/ruxum).

### Reporting Issues

When reporting issues, include:
- The scaffold type (Rust, Next.js, or Full-Stack)
- Your selections in the wizard prompts
- Error message (if any)
- Steps to reproduce

### Contributing New Features

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Test your changes: `cargo run` or the appropriate test command
4. Commit with clear messages
5. Push and open a pull request

---

## License

MIT © 2026 [Andres Fritsche Ogando](https://github.com/AndresFritscheOgando)
