# Template Naming Convention Guide

## Overview
All template files in this project follow **`snake_case`** naming conventions for consistency and predictability.

## Naming Standards

### Template Fragments (`template/fragments/`)
All fragment files use lowercase `snake_case`:

```
✅ CORRECT:
- db_sqlx.rs
- db_drizzle.ts
- auth_middleware.rs
- nextauth_route.ts
- error_handling_examples.rs
- jest_config.ts
- drizzle_schema.ts

❌ INCORRECT:
- DbSqlx.rs (PascalCase)
- DB_SQLX.rs (UPPERCASE)
- dbSqlx.rs (camelCase)
- ErrorHandlingExamples.rs (PascalCase)
```

### Source Template Modules (`src/templates/`)
Source files that generate templates use `snake_case` with language/file-type suffixes:

```
✅ CORRECT:
- main_rs.rs (generates main.rs)
- router_rs.rs (generates router.rs)
- package_json.rs (generates package.json)
- tsconfig_json.rs (generates tsconfig.json)
- next_config.rs (generates next.config.mjs)

Pattern: [file_name]_[extension].rs
```

### Base Template Files (`template/nextjs/base/`, `template/rust/base/`)
Keep actual framework file names as-is (these are the real files):

```
✅ CORRECT:
- package.json (npm standard)
- tsconfig.json (TypeScript standard)
- Cargo.toml (Rust standard)
- next.config.mjs (Next.js standard)
- .env.example (environment standard)
- layout.tsx (React component)
- page.tsx (Next.js page route)
```

## Rationale

- **Consistency**: All non-standard filenames follow a single convention
- **Readability**: Snake_case improves readability across language barriers (works in Rust, TypeScript, file systems)
- **Predictability**: Developers can predict file names without searching
- **Compatibility**: Snake_case works uniformly across Unix/Windows file systems

## When Adding New Template Files

1. **Fragment files** → Use `snake_case` with underscores:
   - `db_postgres.ts` ✅
   - `auth_clerk.ts` ✅
   - `validation_zod.ts` ✅

2. **Source template modules** → Use `[filename]_[ext].rs`:
   - `server_env_ts.rs` ✅
   - `middleware_handler_rs.rs` ✅

3. **Framework files** → Keep original names:
   - Don't rename `package.json` to `package_json.ts`
   - Don't rename `.env.example` to `dot_env_example`

## File Organization

```
template/
├── fragments/
│   ├── nextjs/          # All snake_case
│   │   ├── auth.ts
│   │   ├── db_drizzle.ts
│   │   └── jest_config.ts
│   └── rust/            # All snake_case
│       ├── auth_middleware.rs
│       ├── db_sqlx.rs
│       └── error_handling_examples.rs
├── nextjs/base/         # Real npm/Next.js structure
├── rust/base/           # Real Rust structure
```

## Migration Checklist

When renaming files:
- [ ] Rename the template file to snake_case
- [ ] Search codebase for references to old name
- [ ] Update any hardcoded file paths in Rust code
- [ ] Update any documentation that references the file
- [ ] Test template generation to ensure correct output
- [ ] Commit with message: `refactor: standardize template naming to snake_case`
