import flowbitePlugin from 'flowbite/plugin'
import flowbiteTypographyPlugin from 'flowbite-typography'

/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './src/**/*.{html,js,svelte,ts}', './node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}',
  ],
  darkMode: 'selector',
  theme: {
    extend: {},
  },
  plugins: [
      flowbitePlugin,
      flowbiteTypographyPlugin({
          wysiwyg: true,
      }),
  ],
};

