import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import react from '@astrojs/react';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
	vite: {
		plugins: [tailwindcss()],
	},
	integrations: [
		starlight({
			title: 'create-ruxum-app',
			customCss: ['./src/styles/custom.css'],
			social: [
				{
					icon: 'github',
					label: 'GitHub',
					href: 'https://github.com/AndresFritscheOgando/ruxum',
				},
			],
			sidebar: [
				{
					label: 'Getting Started',
					items: [
						{ label: 'Introduction', slug: 'introduction' },
						{ label: 'Installation', slug: 'installation' },
					],
				},
				{
					label: 'Scaffold Types',
					items: [
						{ label: 'Rust Axum API', slug: 'scaffold/rust' },
						{ label: 'Next.js App', slug: 'scaffold/nextjs' },
						{ label: 'Full-stack', slug: 'scaffold/fullstack' },
					],
				},
				{
					label: 'Configuration',
					items: [
						{ label: 'Database', slug: 'configuration/database' },
						{ label: 'Authentication', slug: 'configuration/authentication' },
					],
				},
			],
		}),
		react(),
	],
});
