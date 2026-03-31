import * as path from "path";
import * as fse from "fs-extra";
import { ScaffoldConfig, RustConfig, NextjsConfig } from "./types";
import { ensureDir, writeProjectFile } from "./helpers/fs";
import { logStep, logSuccess } from "./helpers/logger";

// Rust templates
import { buildCargoToml } from "./templates/cargo";
import { buildMainRs } from "./templates/main_rs";
import { buildRouterRs } from "./templates/router";
import { buildConfigRs } from "./templates/config";
import { buildErrorsRs } from "./templates/errors";
import { buildHealthRs, buildHandlersMod } from "./templates/health";
import { buildDbSqlxRs } from "./templates/db_sqlx";
import { buildDbSeaOrmRs } from "./templates/db_seaorm";
import { buildDbDieselRs } from "./templates/db_diesel";
import { buildAuthMod, buildAuthMiddlewareRs } from "./templates/auth";
import { buildEnvExample as buildRustEnv } from "./templates/env";
import { buildModelsMod } from "./templates/models";

// Next.js templates
import {
  buildPackageJson,
  buildTsConfig,
  buildLayoutTsx,
  buildPageTsx,
  buildNextGitignore,
  buildNextEnvExample,
} from "./templates/nextjs";

export async function scaffoldProject(config: ScaffoldConfig): Promise<void> {
  const { scaffoldType, projectDir } = config;

  await ensureDir(projectDir);

  if (scaffoldType === "rust") {
    await scaffoldRust(projectDir, config.rust!, config);
  } else if (scaffoldType === "nextjs") {
    await scaffoldNextjs(projectDir, config.nextjs!);
  } else if (scaffoldType === "fullstack") {
    const apiDir = path.join(projectDir, "api");
    const wwwDir = path.join(projectDir, "www");

    await ensureDir(apiDir);
    await ensureDir(wwwDir);

    await scaffoldRust(apiDir, config.rust!, config);
    await scaffoldNextjs(wwwDir, config.nextjs!);

    // Add a root README
    const readme = `# ${config.projectName}\n\nFull-stack project scaffolded by create-ruxum-app.\n\n## Structure\n- \`api/\` - Rust Axum backend\n- \`www/\` - Next.js frontend\n`;
    await fse.writeFile(path.join(projectDir, "README.md"), readme);
  }

  logSuccess("Project scaffolded successfully!");
}

async function scaffoldRust(dir: string, cfg: RustConfig, full: ScaffoldConfig) {
  const hasDb = cfg.db !== "none";

  await writeProjectFile(dir, "Cargo.toml", buildCargoToml(full));
  await writeProjectFile(dir, "src/main.rs", buildMainRs(full));
  await writeProjectFile(dir, "src/router.rs", buildRouterRs(full));
  await writeProjectFile(dir, "src/config.rs", buildConfigRs(full));
  await writeProjectFile(dir, "src/errors.rs", buildErrorsRs(full));
  await writeProjectFile(dir, "src/handlers/mod.rs", buildHandlersMod(full));
  await writeProjectFile(dir, "src/handlers/health.rs", buildHealthRs(full));

  if (hasDb) {
    const dbContent = cfg.db.startsWith("sqlx")
      ? buildDbSqlxRs(full)
      : cfg.db.startsWith("diesel")
      ? buildDbDieselRs(full)
      : buildDbSeaOrmRs(full);
    await writeProjectFile(dir, "src/db.rs", dbContent);
    await writeProjectFile(dir, "src/models/mod.rs", buildModelsMod(full));

    if (cfg.db.startsWith("diesel")) {
      await writeProjectFile(
        dir,
        "migrations/00000000000000_initial/up.sql",
        "-- Initial migration\n-- Add your schema here\n"
      );
      await writeProjectFile(
        dir,
        "migrations/00000000000000_initial/down.sql",
        "-- Rollback initial migration\n"
      );
    }
  }

  if (cfg.auth) {
    await writeProjectFile(dir, "src/auth/mod.rs", buildAuthMod(full));
    await writeProjectFile(dir, "src/auth/middleware.rs", buildAuthMiddlewareRs(full));
  }

  await writeProjectFile(dir, ".env.example", buildRustEnv(full));
  logStep("Rust backend scaffolded");
}

async function scaffoldNextjs(dir: string, cfg: NextjsConfig) {
  await ensureDir(path.join(dir, "src/app"));
  await ensureDir(path.join(dir, "src/lib"));

  await writeProjectFile(dir, "package.json", buildPackageJson(cfg));
  await writeProjectFile(dir, "tsconfig.json", buildTsConfig());
  await writeProjectFile(dir, "src/app/layout.tsx", buildLayoutTsx(cfg));
  await writeProjectFile(dir, "src/app/page.tsx", buildPageTsx());
  await writeProjectFile(dir, "src/app/globals.css", "/* Tailwind imports would go here */\n");
  await writeProjectFile(dir, ".gitignore", buildNextGitignore());
  await writeProjectFile(dir, ".env.example", buildNextEnvExample(cfg));

  logStep("Next.js frontend scaffolded");
}
