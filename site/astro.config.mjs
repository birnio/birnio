// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import lucode from 'lucode-starlight';

// https://astro.build/config
export default defineConfig({
	site: 'https://birnio.dev',
	integrations: [
		starlight({
			title: 'Birnio',
			customCss: ['./src/styles/lucode.css'],
			social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/birnio/birnio' }],
			plugins: [
				lucode({
					navLinks: [
						{ label: 'Docs', link: '/docs/' },
						{ label: 'GitHub', link: 'https://github.com/birnio/birnio' },
					],
					footerText: 'Birnio is open source under MIT OR Apache-2.0.',
				}),
			],
			sidebar: [
				{
					label: 'Start',
					items: [
						{ label: 'Overview', slug: 'docs' },
						{ label: 'Installation', slug: 'docs/installation' },
					],
				},
				{
					label: 'Project',
					items: [
						{ label: 'Architecture', slug: 'docs/architecture' },
						{ label: 'Packaging', slug: 'docs/packaging' },
					],
				},
			],
		}),
	],
});
