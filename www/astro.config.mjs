import { defineConfig } from 'astro/config';
import sitemap from '@astrojs/sitemap';
import starlight from '@astrojs/starlight';
import react from '@astrojs/react';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
	site: 'https://create-ruxum-app.vercel.app',
	vite: {
		plugins: [tailwindcss()],
	},
	integrations: [
		sitemap({
			serialize(item) {
				const lastmod = new Date().toISOString().split('T')[0];
				const isHome = item.url === 'https://create-ruxum-app.vercel.app/';
				return {
					...item,
					lastmod,
					changefreq: isHome ? 'monthly' : 'weekly',
					priority: isHome ? 1.0 : 0.8,
				};
			},
		}),
		starlight({
			title: 'create-ruxum-app',
			customCss: ['./src/styles/custom.css'],
			head: [
				{ tag: 'meta', attrs: { name: 'theme-color', content: '#0a0a0a' } },
			],
			components: {
				Head: './src/components/DocHead.astro',
			},
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
