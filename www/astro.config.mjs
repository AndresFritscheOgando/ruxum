import { defineConfig } from 'astro/config';
import sitemap from '@astrojs/sitemap';
import mdx from '@astrojs/mdx';
import react from '@astrojs/react';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
	site: 'https://create-ruxum-app.vercel.app',
	output: 'static',
	vite: {
		plugins: [tailwindcss()],
	},
	integrations: [
		mdx(),
		react(),
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
	],
});
