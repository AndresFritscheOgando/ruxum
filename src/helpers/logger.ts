import chalk from "chalk";

const BANNER = `
  ${chalk.cyan("██████╗ ██╗   ██╗██╗  ██╗██╗   ██╗███╗   ███╗")}
  ${chalk.cyan("██╔══██╗██║   ██║╚██╗██╔╝██║   ██║████╗ ████║")}
  ${chalk.cyan("██████╔╝██║   ██║ ╚███╔╝ ██║   ██║██╔████╔██║")}
  ${chalk.cyan("██╔══██╗██║   ██║ ██╔██╗ ██║   ██║██║╚██╔╝██║")}
  ${chalk.cyan("██║  ██║╚██████╔╝██╔╝ ██╗╚██████╔╝██║ ╚═╝ ██║")}
  ${chalk.cyan("╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚═╝     ╚═╝")}

  ${chalk.dim("Scaffold a production-ready Rust Axum API")}
`;

export function printBanner(): void {
  console.log(BANNER);
}

export function logStep(message: string): void {
  console.log(chalk.cyan("◇ ") + message);
}

export function logSuccess(message: string): void {
  console.log(chalk.green("✔ ") + chalk.bold(message));
}

export function logError(message: string): void {
  console.error(chalk.red("✖ ") + message);
}

export function printNextSteps(projectName: string): void {
  console.log();
  console.log(chalk.green("  ✔ Project scaffolded successfully!"));
  console.log();
  console.log("  Next steps:");
  console.log(chalk.cyan(`    cd ${projectName}`));
  console.log(chalk.cyan("    cp .env.example .env"));
  console.log(chalk.cyan("    cargo run"));
  console.log();
}
