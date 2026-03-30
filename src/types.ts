export type DbChoice =
  | "none"
  | "sqlx-postgres"
  | "sqlx-mysql"
  | "sqlx-sqlite"
  | "seaorm-postgres"
  | "seaorm-mysql";

export interface ScaffoldConfig {
  projectName: string;
  projectDir: string;
  db: DbChoice;
  auth: boolean;
}
