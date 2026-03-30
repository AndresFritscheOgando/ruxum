import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import react from '@astrojs/react';
import tailwind from '@astrojs/tailwind';

// https://astro.build/config
export default defineConfig({
	integrations: [
		starlight({
			title: 'create-ruxum-app',
			defaultLocale: 'es',
			locales: {
				es: {
					label: 'Español',
					lang: 'es',
				},
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
					label: 'Comenzar',
					items: [
						{ label: 'Introducción', slug: 'intro' },
						{ label: 'Instalación', slug: 'instalacion' },
					],
				},
				{
					label: 'Guías',
					autogenerate: { directory: 'guias' },
				},
			],
		}),
		react(),
		tailwind(),
	],
});
