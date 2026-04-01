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
];
