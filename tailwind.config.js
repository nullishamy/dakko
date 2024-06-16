import ctp from '@catppuccin/tailwindcss';
import forms from '@tailwindcss/forms';

/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {}
	},
	plugins: [
		ctp({
			defaultFlavour: 'macchiato'
		}),
		forms()
	]
};
