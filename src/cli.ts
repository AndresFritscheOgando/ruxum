import * as p from "@clack/prompts";
import * as path from "path";
import * as fse from "fs-extra";
import chalk from "chalk";
import {
  ScaffoldConfig,
  ScaffoldType,
  DbChoice,
  OrmChoice,
  DbProvider,
  RustConfig,
  NextjsConfig,
} from "./types";
import { printBanner } from "./helpers/logger";

function isValidProjectName(name: string): boolean {
  return /^[a-zA-Z0-9_-]+$/.test(name);
}

export async function runWizard(): Promise<ScaffoldConfig> {
  printBanner();

  p.intro(chalk.bold("Create a new Ruxum (Rust + Axum + Next.js) project"));

  // 1. Scaffold type
  const scaffoldType = (await p.select({
    message: "What would you like to scaffold?",
    options: [
      { value: "rust", label: "Rust Axum API" },
      { value: "nextjs", label: "Next.js App" },
      { value: "fullstack", label: "Full-stack (Axum + Next.js)" },
    ],
  })) as ScaffoldType;

  if (p.isCancel(scaffoldType)) {
    p.cancel("Operation cancelled.");
    process.exit(0);
  }

  // 2. Project name
  const projectName = await p.text({
    message: "What is your project named?",
    defaultValue: "my-app",
    placeholder: "my-app",
    validate(value) {
      const name = value || "my-app";
      if (!isValidProjectName(name)) {
        return "Project name must only contain letters, numbers, hyphens, and underscores.";
      }
    },
  });

  if (p.isCancel(projectName)) {
    p.cancel("Operation cancelled.");
    process.exit(0);
  }

  const name = (projectName as string) || "my-app";
  const projectDir = path.resolve(process.cwd(), name);

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

  // 3. Rust Config
  let rust: RustConfig | undefined;
  if (scaffoldType === "rust" || scaffoldType === "fullstack") {
    const db = (await p.select({
      message: "Database (Rust Axum)",
      options: [
        { value: "none", label: "None" },
        { value: "sqlx-postgres", label: "SQLx — PostgreSQL" },
        { value: "sqlx-mysql", label: "SQLx — MySQL" },
        { value: "sqlx-sqlite", label: "SQLx — SQLite" },
        { value: "seaorm-postgres", label: "SeaORM — PostgreSQL" },
        { value: "seaorm-mysql", label: "SeaORM — MySQL" },
      ],
    })) as DbChoice;

    if (p.isCancel(db)) {
      p.cancel("Operation cancelled.");
      process.exit(0);
    }

    const auth = await p.confirm({
      message: "Add JWT authentication to Rust backend?",
      initialValue: false,
    });

    if (p.isCancel(auth)) {
      p.cancel("Operation cancelled.");
      process.exit(0);
    }

    rust = { db, auth: auth as boolean };
  }

  // 4. Next.js Config
  let nextjs: NextjsConfig | undefined;
  if (scaffoldType === "nextjs" || scaffoldType === "fullstack") {
    const tailwind = await p.confirm({
      message: "Add Tailwind CSS?",
      initialValue: true,
    });

    if (p.isCancel(tailwind)) {
      p.cancel("Operation cancelled.");
      process.exit(0);
    }

    let shadcn = false;
    if (tailwind) {
      const shadcnConfirm = await p.confirm({
        message: "Add shadcn/ui?",
        initialValue: true,
      });
      if (p.isCancel(shadcnConfirm)) {
        p.cancel("Operation cancelled.");
        process.exit(0);
      }
      shadcn = shadcnConfirm as boolean;
    }

    const orm = (await p.select({
      message: "Database ORM (Next.js)",
      options: [
        { value: "none", label: "None" },
        { value: "prisma", label: "Prisma" },
        { value: "drizzle", label: "Drizzle" },
      ],
    })) as OrmChoice;

    if (p.isCancel(orm)) {
      p.cancel("Operation cancelled.");
      process.exit(0);
    }

    let dbProvider: DbProvider = "postgres";
    if (orm !== "none") {
      dbProvider = (await p.select({
        message: "Database Provider (Next.js)",
        options: [
          { value: "postgres", label: "PostgreSQL" },
          { value: "mysql", label: "MySQL" },
          { value: "sqlite", label: "SQLite" },
        ],
      })) as DbProvider;

      if (p.isCancel(dbProvider)) {
        p.cancel("Operation cancelled.");
        process.exit(0);
      }
    }

    const nextAuth = await p.confirm({
      message: "Add NextAuth.js?",
      initialValue: false,
    });

    if (p.isCancel(nextAuth)) {
      p.cancel("Operation cancelled.");
      process.exit(0);
    }

    const jest = await p.confirm({
      message: "Add Jest + React Testing Library?",
      initialValue: false,
    });

    if (p.isCancel(jest)) {
      p.cancel("Operation cancelled.");
      process.exit(0);
    }

    const rspc = await p.confirm({
      message: "Add rspc (type-safe API for Rust + Next.js)?",
      initialValue: true,
    });

    if (p.isCancel(rspc)) {
      p.cancel("Operation cancelled.");
      process.exit(0);
    }

    nextjs = {
      tailwind: tailwind as boolean,
      shadcn,
      orm,
      dbProvider,
      nextAuth: nextAuth as boolean,
      jest: jest as boolean,
      rspc: rspc as boolean,
    };
  }

  // 5. Run install
  const runInstall = await p.confirm({
    message: "Run package managers after scaffolding?",
    initialValue: true,
  });

  if (p.isCancel(runInstall)) {
    p.cancel("Operation cancelled.");
    process.exit(0);
  }

  // Summary
  console.log();
  p.note(
    `Project: ${chalk.bold(name)}\n` +
      `Type:    ${chalk.bold(scaffoldType)}\n` +
      (rust ? `Rust DB: ${chalk.bold(rust.db)}\n` : "") +
      (nextjs ? `Next ORM: ${chalk.bold(nextjs.orm)}\n` : ""),
    "Summary"
  );

  const confirmed = await p.confirm({
    message: "Scaffold this project?",
    initialValue: true,
  });

  if (p.isCancel(confirmed) || !confirmed) {
    p.cancel("Operation cancelled.");
    process.exit(0);
  }

  return {
    projectName: name,
    projectDir,
    scaffoldType,
    rust,
    nextjs,
    runInstall: runInstall as boolean,
  };
}
