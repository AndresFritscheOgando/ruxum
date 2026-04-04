import { z } from "zod";

const envSchema = z.object({
  // Auth (next-auth)
  AUTH_GITHUB_ID: z.string().min(1, "AUTH_GITHUB_ID is required"),
  AUTH_GITHUB_SECRET: z.string().min(1, "AUTH_GITHUB_SECRET is required"),
  AUTH_SECRET: z.string().min(1, "AUTH_SECRET is required for production"),

  // Optional: Add other env vars as needed
  NODE_ENV: z.enum(["development", "production", "test"]).default("development"),
});

type Env = z.infer<typeof envSchema>;

let validatedEnv: Env;

try {
  validatedEnv = envSchema.parse(process.env);
} catch (error) {
  if (error instanceof z.ZodError) {
    const missingVars = error.errors
      .map((err) => `${err.path.join(".")}: ${err.message}`)
      .join("\n");

    throw new Error(
      `Environment variable validation failed:\n${missingVars}\n\nMake sure all required variables are set in your .env.local file.`
    );
  }
  throw error;
}

export const env = validatedEnv;
