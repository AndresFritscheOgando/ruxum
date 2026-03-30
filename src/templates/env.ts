import { ScaffoldConfig } from "../types";

export function buildEnvExample(config: ScaffoldConfig): string {
  const db = config.rust?.db;
  const auth = config.rust?.auth;
  const hasDb = db && db !== "none";

  let env = `HOST=127.0.0.1
PORT=3000
`;

  if (hasDb) {
    if (db === "sqlx-postgres" || db === "seaorm-postgres") {
      env += `DATABASE_URL=postgres://user:password@localhost/mydb\n`;
    } else if (db === "sqlx-mysql" || db === "seaorm-mysql") {
      env += `DATABASE_URL=mysql://user:password@localhost/mydb\n`;
    } else if (db === "sqlx-sqlite") {
      env += `DATABASE_URL=sqlite://./dev.db\n`;
    }
  }

  if (auth) {
    env += `JWT_SECRET=changeme_use_a_long_random_string_in_production\n`;
  }

  return env;
}
