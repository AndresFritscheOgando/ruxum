---
title: Installation
description: Get your first ruxum project running in under a minute.
---

## Prerequisites

| Requirement | Minimum version |
|---|---|
| Node.js | 18.0.0 |
| Rust + Cargo | stable (latest recommended) |

Install Rust via [rustup.rs](https://rustup.rs) if you don't have it yet.

## Quick start

Run the interactive wizard with `npx`:

```sh
npx create-ruxum-app@latest
```

The wizard will ask you:

1. **What to scaffold** — Rust API, Next.js App, or Full-stack
2. **Project name** — used as the directory name
3. **Database** *(Rust only)* — PostgreSQL, MySQL, or SQLite
4. **ORM** *(Rust only)* — SQLx or SeaORM
5. **JWT authentication** *(Rust only)* — optional
6. **Next.js extras** *(Next.js only)* — TypeScript, Tailwind CSS

## Running a Rust Axum project

```sh
cd my-app

# Copy and fill in your environment variables
cp .env.example .env

# Start the server
cargo run
```

Your API will be listening at `http://127.0.0.1:3000`.

Verify it's running:

```sh
curl http://127.0.0.1:3000/health
# {"status":"ok","version":"0.1.0"}
```

## Running a Next.js project

```sh
cd my-app
npm install
npm run dev
```

Your app will be at `http://localhost:3000`.

## Running a full-stack project

```sh
cd my-app

# Terminal 1 — Rust API
cp api/.env.example api/.env
cd api && cargo run

# Terminal 2 — Next.js frontend
cd www && npm install && npm run dev
```

The API runs on port `3000`, the frontend on port `3001` by default.
