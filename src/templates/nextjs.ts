import { NextjsConfig, DbProvider, OrmChoice } from "../types";

export const buildPackageJson = (cfg: NextjsConfig): string => {
  const hasOrm = cfg.orm !== "none";

  const deps: Record<string, string> = {
    next: "15.1.0",
    react: "19.0.0",
    "react-dom": "19.0.0",
  };

  if (cfg.nextAuth) {
    deps["next-auth"] = "5.0.0-beta.25";
  }

  if (cfg.orm === "prisma") {
    deps["@prisma/client"] = "6.0.1";
  }

  if (cfg.orm === "drizzle") {
    deps["drizzle-orm"] = "0.38.2";
    if (cfg.dbProvider === "postgres") deps["postgres"] = "3.4.5";
    if (cfg.dbProvider === "mysql") deps["mysql2"] = "3.12.0";
    if (cfg.dbProvider === "sqlite") deps["better-sqlite3"] = "11.7.0";
  }

  const devDeps: Record<string, string> = {
    typescript: "5.7.2",
    "@types/node": "22.10.2",
    "@types/react": "19.0.1",
    "@types/react-dom": "19.0.2",
    eslint: "9.17.0",
    "eslint-config-next": "15.1.0",
  };

  if (cfg.tailwind) {
    devDeps["tailwindcss"] = "3.4.16";
    devDeps["postcss"] = "8.4.49";
    devDeps["autoprefixer"] = "10.4.20";
  }

  if (cfg.orm === "prisma") devDeps["prisma"] = "6.0.1";
  if (cfg.orm === "drizzle") devDeps["drizzle-kit"] = "0.30.1";

  if (cfg.jest) {
    devDeps["jest"] = "29.7.0";
    devDeps["jest-environment-jsdom"] = "29.7.0";
    devDeps["@testing-library/react"] = "16.1.0";
    devDeps["@testing-library/jest-dom"] = "6.6.3";
    devDeps["@types/jest"] = "29.5.14";
    devDeps["ts-jest"] = "29.2.5";
  }

  const scripts: Record<string, string> = {
    dev: "next dev",
    build: "next build",
    start: "next start",
    lint: "next lint",
  };

  if (hasOrm) {
    if (cfg.orm === "prisma") {
      scripts["db:push"] = "prisma db push";
      scripts["db:studio"] = "prisma studio";
    } else {
      scripts["db:push"] = "drizzle-kit push";
      scripts["db:studio"] = "drizzle-kit studio";
    }
  }

  if (cfg.jest) {
    scripts["test"] = "jest";
    scripts["test:watch"] = "jest --watch";
  }

  return JSON.stringify(
    {
      name: "my-app",
      version: "0.1.0",
      private: true,
      scripts,
      dependencies: deps,
      devDependencies: devDeps,
    },
    null,
    2
  );
};

export const buildTsConfig = (): string => `{
  "compilerOptions": {
    "target": "ES2017",
    "lib": ["dom", "dom.iterable", "esnext"],
    "allowJs": true,
    "skipLibCheck": true,
    "strict": true,
    "noEmit": true,
    "esModuleInterop": true,
    "module": "esnext",
    "moduleResolution": "bundler",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "jsx": "preserve",
    "incremental": true,
    "plugins": [{ "name": "next" }],
    "paths": {
      "@/*": ["./src/*"]
    }
  },
  "include": ["next-env.d.ts", "**/*.ts", "**/*.tsx", ".next/types/**/*.ts"],
  "exclude": ["node_modules"]
}`;

export const buildLayoutTsx = (cfg: NextjsConfig): string => `import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Create Ruxum App",
  description: "Scaffolded with create-ruxum-app",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className="antialiased">{children}</body>
    </html>
  );
}
`;

export const buildPageTsx = (): string => `export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-center bg-gradient-to-b from-[#2e026d] to-[#15162c] text-white">
      <div className="container flex flex-col items-center justify-center gap-12 px-4 py-16">
        <h1 className="text-5xl font-extrabold tracking-tight sm:text-[5rem]">
          Create <span className="text-[hsl(280,100%,70%)]">Ruxum</span> App
        </h1>
        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 md:gap-8">
          <div className="flex max-w-xs flex-col gap-4 rounded-xl bg-white/10 p-4 hover:bg-white/20">
            <h3 className="text-2xl font-bold">Documentation →</h3>
            <div className="text-lg">
              Learn more about Ruxum and how to build high-performance APIs.
            </div>
          </div>
          <div className="flex max-w-xs flex-col gap-4 rounded-xl bg-white/10 p-4 hover:bg-white/20">
            <h3 className="text-2xl font-bold">Next.js Frontend →</h3>
            <div className="text-lg">
              Explore the Next.js frontend and how it integrates with Axum.
            </div>
          </div>
        </div>
      </div>
    </main>
  );
}
`;

export const buildNextGitignore = (): string => `# Next.js
.next/
out/
build/

# production
dist/

# misc
.DS_Store
*.pem

# debug
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# local env files
.env*.local

# vercel
.vercel

# typescript
*.tsbuildinfo
next-env.d.ts
node_modules/
`;

export const buildNextEnvExample = (cfg: NextjsConfig): string => {
  let s = "";
  if (cfg.orm !== "none") {
    if (cfg.dbProvider === "sqlite") s += "DATABASE_URL=file:./dev.db\n";
    else s += "DATABASE_URL=postgresql://user:password@localhost:5432/mydb\n";
  }
  if (cfg.nextAuth) {
    s += "NEXTAUTH_SECRET=changeme\nNEXTAUTH_URL=http://localhost:3000\n";
  }
  return s || "# No env vars needed yet\n";
};
