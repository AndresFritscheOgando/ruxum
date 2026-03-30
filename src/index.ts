#!/usr/bin/env node
import { runWizard } from "./cli";
import { scaffoldProject } from "./scaffold";
import { printNextSteps, logError } from "./helpers/logger";

async function runCLI(): Promise<void> {
  try {
    const config = await runWizard();
    await scaffoldProject(config);
    printNextSteps(config.projectName);
  } catch (err) {
    logError(err instanceof Error ? err.message : String(err));
    process.exit(1);
  }
}

runCLI();
