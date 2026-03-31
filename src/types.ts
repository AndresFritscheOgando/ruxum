export type ScaffoldType = "rust" | "nextjs" | "fullstack";

export type DbChoice =
  | "none"
  | "sqlx-postgres"
  | "sqlx-mysql"
  | "sqlx-sqlite"
  | "seaorm-postgres"
  | "seaorm-mysql"
  | "diesel-postgres"
  | "diesel-mysql"
  | "diesel-sqlite";

export type OrmChoice = "none" | "prisma" | "drizzle";

export type DbProvider = "postgres" | "mysql" | "sqlite";

export interface RustConfig {
  db: DbChoice;
  auth: boolean;
}

export interface NextjsConfig {
  tailwind: boolean;
  shadcn: boolean;
  orm: OrmChoice;
  dbProvider: DbProvider;
  nextAuth: boolean;
  jest: boolean;
  rspc: boolean;
}

export interface ScaffoldConfig {
  projectName: string;
  projectDir: string;
  scaffoldType: ScaffoldType;
  rust?: RustConfig;
  nextjs?: NextjsConfig;
  runInstall: boolean;
}
