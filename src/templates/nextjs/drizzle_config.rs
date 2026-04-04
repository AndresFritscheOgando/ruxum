import type { Config } from "drizzle-kit";

export default {
  schema: "./drizzle/schema.ts",
  out: "./drizzle/migrations",
  dialect: "{{dialect}}",
  dbCredentials: {
    url: process.env.DATABASE_URL!,
  },
} satisfies Config;
