import { defineConfig } from 'astro/config';
import sitemap from '@astrojs/sitemap';
import mdx from '@astrojs/mdx';
import react from '@astrojs/react';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
	site: 'https://ruxum.dev',
	output: 'static',
	vite: {
		plugins: [tailwindcss()],
		server: {
			allowedHosts: ['ruxum.dev', 'localhost', '127.0.0.1'],
		},
	},
	integrations: [
		mdx(),
		react(),
		sitemap({
			serialize(item) {
				const lastmod = new Date().toISOString().split('T')[0];
				const isHome = item.url === 'https://ruxum.dev/';
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
