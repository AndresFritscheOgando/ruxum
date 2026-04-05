export interface NavItem {
  label: string;
  slug: string;
}

export interface NavGroup {
  title: string;
  items: NavItem[];
}

export const SIDEBAR_NAV: NavGroup[] = [
  {
    title: 'Getting Started',
    items: [
      { label: 'Introduction',  slug: 'introduction'  },
      { label: 'Quick Install', slug: 'installation'  },
      { label: 'Why Ruxum?',    slug: 'why-ruxum'     },
      { label: 'Running Locally', slug: 'getting-started/running-locally' },
    ],
  },
  {
    title: 'Project Structure',
    items: [
      { label: 'Folder Structure', slug: 'folder-structure' },
      { label: 'First Steps',      slug: 'first-steps'      },
    ],
  },
  {
    title: 'Scaffold Types',
    items: [
      { label: 'Next.js App', slug: 'scaffold/nextjs'   },
      { label: 'Rust API',    slug: 'scaffold/rust'     },
      { label: 'Full-stack',  slug: 'scaffold/fullstack'},
    ],
  },
  {
    title: 'Configuration',
    items: [
      { label: 'Database',       slug: 'configuration/database'       },
      { label: 'Authentication', slug: 'configuration/authentication' },
    ],
  },
  {
    title: 'Development',
    items: [
      { label: 'API Patterns',  slug: 'development/api-patterns'  },
      { label: 'Testing',       slug: 'development/testing'       },
    ],
  },
  {
    title: 'Deployment',
    items: [
      { label: 'Railway', slug: 'deployment/railway' },
    ],
  },
  {
    title: 'Guides',
    items: [
      { label: 'Full-Stack Integration', slug: 'guides/full-stack' },
      { label: 'Frontend Patterns',     slug: 'guides/frontend-patterns' },
      { label: 'Security',              slug: 'guides/security' },
    ],
  },
  {
    title: 'Examples',
    items: [
      { label: 'Todo App', slug: 'examples/todo-app' },
    ],
  },
  {
    title: 'Reference',
    items: [
      { label: 'Troubleshooting', slug: 'reference/troubleshooting' },
      { label: 'FAQ',             slug: 'reference/faq'             },
    ],
  },
];
