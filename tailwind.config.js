import flowbitePlugin from 'flowbite/plugin'
import flowbiteTypographyPlugin from 'flowbite-typography'

/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './src/**/*.{html,js,svelte,ts}', './node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}',
  ],
  darkMode: 'selector',
  theme: {
    colors: {
      text: "hsl(var(--color-text))",
      subtext2: "hsl(var(--color-subtext2))",
      subtext1: "hsl(var(--color-subtext1))",
      subtext0: "hsl(var(--color-subtext0))",
      overlay2: "hsl(var(--color-overlay2))",
      overlay1: "hsl(var(--color-overlay1))",
      overlay0: "hsl(var(--color-overlay0))",
      surface2: "hsl(var(--color-surface2))",
      surface1: "hsl(var(--color-surface1))",
      surface0: "hsl(var(--color-surface0))",
      base: "hsl(var(--color-base))",
      crust: "hsl(var(--color-crust))",
      mantle: "hsl(var(--color-mantle))",
    },
    extend: {},
  },
  plugins: [
      flowbitePlugin,
      flowbiteTypographyPlugin({
          wysiwyg: true,
      }),
  ],
};

