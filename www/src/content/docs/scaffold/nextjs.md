---
title: Next.js App
description: Generate a Next.js 15 app with App Router, TypeScript, and optional Tailwind CSS — wired to your Axum API from day one.
---

The Next.js scaffold generates a [Next.js 15](https://nextjs.org) application with App Router, TypeScript, and optional Tailwind CSS.

## Project structure

```
my-app/
├── app/
│   ├── layout.tsx        # Root layout
│   ├── page.tsx          # Home page
│   └── globals.css       # Global styles
├── components/           # Shared UI components
├── lib/                  # Utility functions
├── public/               # Static assets
├── package.json
├── tsconfig.json
└── next.config.ts
```

With **Tailwind CSS** enabled, you also get:

```
├── tailwind.config.ts
└── postcss.config.mjs
```

## TypeScript

All files are TypeScript by default. The `tsconfig.json` uses `strict: true` and path aliases:

```ts
// Import from anywhere using the @ alias
import { Button } from "@/components/ui/button";
```

## Tailwind CSS

Tailwind v4 is configured with the default palette. Customize it in `tailwind.config.ts` or via CSS variables in `globals.css`.

## Adding a page

Create a directory with a `page.tsx` inside `app/`:

```tsx
// app/about/page.tsx
export default function AboutPage() {
  return <main>About</main>;
}
```

## Fetching data

The scaffold uses Next.js Server Components by default:

```tsx
// app/posts/page.tsx
async function getPosts() {
  const res = await fetch("http://localhost:3000/posts");
  return res.json();
}

export default async function PostsPage() {
  const posts = await getPosts();
  return <ul>{posts.map(p => <li key={p.id}>{p.title}</li>)}</ul>;
}
```
