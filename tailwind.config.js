/** @type {import('tailwindcss').Config} */

export default {
  content: [
    "./src/**/*.{html,js,svelte,ts}",
    "./node_modules/flowbite/**/*.js",
  ],
  theme: {
    colors: {
      text: "var(--color-text)",
      subtext2: "var(--color-subtext2)",
      subtext1: "var(--color-subtext1)",
      subtext0: "var(--color-subtext0)",
      overlay2: "var(--color-overlay2)",
      overlay1: "var(--color-overlay1)",
      overlay0: "var(--color-overlay0)",
      surface2: "var(--color-surface2)",
      surface1: "var(--color-surface1)",
      surface0: "var(--color-surface0)",
      base: "var(--color-base)",
      crust: "var(--color-crust)",
      mantle: "var(--color-mantle)",
    },
    extend: {},
  },
  plugins: [
    require("flowbite-typography"),
    require("flowbite/plugin")({
      wysiwyg: true,
    }),
  ],
};
