import * as p from "@clack/prompts";
import * as path from "path";
import * as fse from "fs-extra";
import chalk from "chalk";
import { ScaffoldConfig, DbChoice } from "./types";
import { printBanner, logError } from "./helpers/logger";

function isValidProjectName(name: string): boolean {
  return /^[a-zA-Z0-9_-]+$/.test(name);
}

export async function runWizard(): Promise<ScaffoldConfig> {
  printBanner();

  p.intro(chalk.bold("Create a new Ruxum (Rust + Axum) project"));

  // Step 1 — Project name
  const projectName = await p.text({
    message: "What is your project named?",
    defaultValue: "my-axum-app",
    placeholder: "my-axum-app",
    validate(value) {
      const name = value || "my-axum-app";
      if (!isValidProjectName(name)) {
        return "Project name must only contain letters, numbers, hyphens, and underscores.";
      }
    },
  });

  if (p.isCancel(projectName)) {
    p.cancel("Operation cancelled.");
    process.exit(0);
  }

  const name = (projectName as string) || "my-axum-app";
  const projectDir = path.resolve(process.cwd(), name);

  // Check if directory already exists
  if (await fse.pathExists(projectDir)) {
    const overwrite = await p.confirm({
      message: `Directory "${name}" already exists. Overwrite?`,
      initialValue: false,
    });

    if (p.isCancel(overwrite) || !overwrite) {
      p.cancel("Operation cancelled.");
      process.exit(0);
    }

    await fse.remove(projectDir);
  }

  // Step 2 — Database
  const dbChoice = await p.select({
    message: "Which database would you like to use?",
    options: [
      { value: "none", label: "None" },
      { value: "sqlx-postgres", label: "SQLx — PostgreSQL" },
      { value: "sqlx-mysql", label: "SQLx — MySQL" },
      { value: "sqlx-sqlite", label: "SQLx — SQLite" },
      { value: "seaorm-postgres", label: "SeaORM — PostgreSQL" },
      { value: "seaorm-mysql", label: "SeaORM — MySQL" },
    ],
  });

  if (p.isCancel(dbChoice)) {
    p.cancel("Operation cancelled.");
    process.exit(0);
  }

  // Step 3 — Auth
  const authChoice = await p.confirm({
    message: "Add JWT authentication? (jsonwebtoken + Bearer extractor)",
    initialValue: false,
  });

  if (p.isCancel(authChoice)) {
    p.cancel("Operation cancelled.");
    process.exit(0);
  }

  const db = dbChoice as DbChoice;
  const auth = authChoice as boolean;

  // Step 4 — Summary + confirm
  const dbLabel: Record<DbChoice, string> = {
    none: "None",
    "sqlx-postgres": "SQLx — PostgreSQL",
    "sqlx-mysql": "SQLx — MySQL",
    "sqlx-sqlite": "SQLx — SQLite",
    "seaorm-postgres": "SeaORM — PostgreSQL",
    "seaorm-mysql": "SeaORM — MySQL",
  };

  console.log();
  console.log(chalk.dim("  ┌─────────────────────────────────┐"));
  console.log(`  ${chalk.dim("│")}  Project:   ${chalk.bold(name.padEnd(19))}${chalk.dim("│")}`);
  console.log(`  ${chalk.dim("│")}  Database:  ${chalk.bold(dbLabel[db].padEnd(19))}${chalk.dim("│")}`);
  console.log(`  ${chalk.dim("│")}  Auth:      ${chalk.bold((auth ? "JWT ✔" : "None").padEnd(19))}${chalk.dim("│")}`);
  console.log(chalk.dim("  └─────────────────────────────────┘"));
  console.log();

  const confirmed = await p.confirm({
    message: "Scaffold this project?",
    initialValue: true,
  });

  if (p.isCancel(confirmed) || !confirmed) {
    p.cancel("Operation cancelled.");
    process.exit(0);
  }

  console.log();

  return {
    projectName: name,
    projectDir,
    db,
    auth,
  };
}
