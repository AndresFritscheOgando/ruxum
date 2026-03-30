import * as path from "path";
import { ScaffoldConfig } from "./types";
import { ensureDir, writeProjectFile } from "./helpers/fs";
import { logStep, logSuccess } from "./helpers/logger";
import { buildCargoToml } from "./templates/cargo";
import { buildMainRs } from "./templates/main_rs";
import { buildRouterRs } from "./templates/router";
import { buildConfigRs } from "./templates/config";
import { buildErrorsRs } from "./templates/errors";
import { buildHealthRs, buildHandlersMod } from "./templates/health";
import { buildDbSqlxRs } from "./templates/db_sqlx";
import { buildDbSeaOrmRs } from "./templates/db_seaorm";
import { buildAuthMod, buildAuthMiddlewareRs } from "./templates/auth";
import { buildEnvExample } from "./templates/env";
import { buildModelsMod } from "./templates/models";

export async function scaffoldProject(config: ScaffoldConfig): Promise<void> {
  const { projectName, projectDir, db, auth } = config;
  const hasDb = db !== "none";

  await ensureDir(projectDir);
  logStep(`Created ${projectName}/`);

  await writeProjectFile(projectDir, "Cargo.toml", buildCargoToml(config));
  logStep("Written Cargo.toml");

  await writeProjectFile(projectDir, "src/main.rs", buildMainRs(config));
  logStep("Written src/main.rs");

  await writeProjectFile(projectDir, "src/router.rs", buildRouterRs(config));
  logStep("Written src/router.rs");

  await writeProjectFile(projectDir, "src/config.rs", buildConfigRs(config));
  logStep("Written src/config.rs");

  await writeProjectFile(projectDir, "src/errors.rs", buildErrorsRs(config));
  logStep("Written src/errors.rs");

  await writeProjectFile(projectDir, "src/handlers/mod.rs", buildHandlersMod(config));
  logStep("Written src/handlers/mod.rs");

  await writeProjectFile(projectDir, "src/handlers/health.rs", buildHealthRs(config));
  logStep("Written src/handlers/health.rs");

  if (hasDb) {
    const dbContent = db.startsWith("sqlx")
      ? buildDbSqlxRs(config)
      : buildDbSeaOrmRs(config);
    await writeProjectFile(projectDir, "src/db.rs", dbContent);
    logStep("Written src/db.rs");

    await writeProjectFile(projectDir, "src/models/mod.rs", buildModelsMod(config));
    logStep("Written src/models/mod.rs");
  }

  if (auth) {
    await writeProjectFile(projectDir, "src/auth/mod.rs", buildAuthMod(config));
    logStep("Written src/auth/mod.rs");

    await writeProjectFile(projectDir, "src/auth/middleware.rs", buildAuthMiddlewareRs(config));
    logStep("Written src/auth/middleware.rs");
  }

  await writeProjectFile(projectDir, ".env.example", buildEnvExample(config));
  logStep("Written .env.example");

  logSuccess("Project scaffolded successfully!");
}
