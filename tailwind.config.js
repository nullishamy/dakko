import ctp from '@catppuccin/tailwindcss'

/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {},
  },
  plugins: [
    ctp({
      defaultFlavour: 'mocha'
    })
  ],
}

