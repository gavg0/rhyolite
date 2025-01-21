import { type Writable, writable, get } from "svelte/store";
import type { Theme } from "../types/theme";
import { defaultTheme } from "$lib/themes/default";
import { greenScreenTheme } from "$lib/themes/greenscreen";
import { coffeeTheme } from "$lib/themes/coffee";
import { catppuccinMacchiatoTheme } from "$lib/themes/catppuccinMacchiato";
// import { rosePaneMainTheme } from "$lib/themes/rosePineMain";

export interface IThemesStates {
  themes: Theme[];
  currentTheme: Theme;
}

const DEFAULT_THEMES: Theme[] = [
<<<<<<< HEAD
  defaultTheme,
  greenScreenTheme,
  coffeeTheme,
  catppuccinMacchiatoTheme,
  // rosePaneMainTheme,
=======
  {
    name: "Default",
    colorscheme: "dark",
    colors: {
      text: "#ffffff",
      subtext2: "#f1f2f3",
      subtext1: "#d8dbde",
      subtext0: "#c2c6cb",
      overlay2: "#acb2b8",
      overlay1: "#969da5",
      overlay0: "#808992",
      surface2: "#6c757d",
      surface1: "#596167",
      surface0: "#464c51",
      base: "#33373b",
      crust: "#202325",
      mantle: "#0d0e0f",
    },
  },
  {
    name: "Green Screen",
    colorscheme: "dark",
    colors: {
      text: "#ffffff",
      subtext2: "#f1f2f3",
      subtext1: "#d8dbde",
      subtext0: "#c2c6cb",
      overlay2: "#acb2b8",
      overlay1: "#969da5",
      overlay0: "#808992",
      surface2: "#609060",
      surface1: "#508010",
      surface0: "#40a040",
      base: "#335f3b",
      crust: "#203325",
      mantle: "#081d08",
    },
  },
  {
    name: "Coffee",
    colorscheme: "light",
    colors: {
      text: "#FFDBB5",
      subtext2: "#FFEAC5",
      subtext1: "#d8dbde",
      subtext0: "#c2c6cb",
      overlay2: "#acb2b8",
      overlay1: "#969da5",
      overlay0: "#808992",
      surface2: "#a07349",
      surface1: "#845f3c",
      surface0: "#684b2f",
      base: "#4c3622",
      crust: "#2f2215",
      mantle: "#130e09",
    },
  },
  {
    name: "Catppuccin Latte",
    colorscheme: "dark",
    colors: {
      text: "#4c4f69",
      subtext2: "#5a5d75",
      subtext1: "#5c5f77",
      subtext0: "#6c6f85",
      overlay2: "#75788c",
      overlay1: "#7c7f93",
      overlay0: "#8c8fa1",
      surface2: "#9ca0b0",
      surface1: "#acb0be",
      surface0: "#bcc0cc",
      base: "#ccd0da",
      crust: "#eff1f5",
      mantle: "#e6e9ef",
    },
  },
  {
    name: "Catppuccin Macchiato",
    colorscheme: "dark",
    colors: {
      text: "#cad3f5",
      subtext2: "#bec5e0",
      subtext1: "#b8c0e0",
      subtext0: "#a5adcb",
      overlay2: "#939ab7",
      overlay1: "#8087a2",
      overlay0: "#6e738d",
      surface2: "#5e6277",
      surface1: "#5b6078",
      surface0: "#494d64",
      base: "#363a4f",
      crust: "#24273a",
      mantle: "#1e2030",
    },
  },
  {
    name: "Catppuccin Mocha",
    colorscheme: "dark",
    colors: {
      text: "#cdd6f4",
      subtext2: "#bec6e0",
      subtext1: "#bac2de",
      subtext0: "#a6adc8",
      overlay2: "#989db5",
      overlay1: "#9399b2",
      overlay0: "#7f849c",
      surface2: "#6c7086",
      surface1: "#585b70",
      surface0: "#45475a",
      base: "#313244",
      crust: "#1e1e2e",
      mantle: "#181825",
    },
  },
>>>>>>> upstream/master
];

const states: Writable<IThemesStates> = writable<IThemesStates>({
  themes: DEFAULT_THEMES,
  currentTheme: DEFAULT_THEMES[0],
});

const initThemesStore = async () => {
  // TODO: pick from some store
  const themes = DEFAULT_THEMES;
  // TODO: use last used theme or from browser colorscheme
  const currentTheme = themes[0];
  applyTheme(currentTheme);
  states.update(() => ({ themes, currentTheme }));
};

const resetCurrentTheme = () => {
  const themes: Theme[] = getThemesState();
  const currentTheme: Theme = themes[0];
  updateCurrentThemeState(currentTheme);
};

const updateThemesState = (themes: Theme[]): Theme[] => {
  states.update((data) => ({
    themes: themes,
    currentTheme: data.currentTheme,
  }));
  return themes;
};

const colorToRgb = (color: string) => {
  let match = /^#([a-f0-9]{2})([a-f0-9]{2})([a-f0-9]{2})$/i.exec(color);
  if (match) {
    return match.slice(1).map((hex) => parseInt(hex, 16));
  }
  match = /^#([a-f0-9])([a-f0-9])([a-f0-9])$/i.exec(color);
  if (match) {
    return match.slice(1).map((hex) => parseInt(hex + hex, 16));
  }
  match =
    /^rgb\(\s*(\d+)(?:\s*,\s*|\s+)(\d+)(?:\s*,\s*|\s+)(\d+)\s*\)\s*$/i.exec(
      color,
    );
  if (match) {
    return match.slice(1).map((num) => parseInt(num));
  }
  throw new Error(`Unsupported color: "${color}"`);
};

const applyTheme = (theme: Theme) => {
  const root: HTMLHtmlElement = document.querySelector(":root")!;
  Object.entries(theme.colors).forEach(([name, value]) => {
    root.style.setProperty(`--color-${name}`, colorToRgb(value).join(" "));
  });
  root.style.setProperty(`--theme-name`, theme.name);
  root.style.setProperty(`--theme-colorscheme`, theme.colorscheme);
};

const updateCurrentThemeState = (currentTheme: Theme): Theme => {
  states.update((data) => ({
    themes: data.themes,
    currentTheme: currentTheme,
  }));
  applyTheme(currentTheme);
  return currentTheme;
};

const getThemeById = (themeName: string): Theme | undefined => {
  const { themes }: { themes: Theme[] } = get(states); // Access the current state of states
  return themes.find((theme) => theme.name === themeName); // Replace 'id' with the actual property name if it's different
};

const getCurrentThemeState = (): Theme => {
  const { currentTheme }: { currentTheme: Theme } = get(states);
  return currentTheme;
};

const getThemesState = (): Theme[] => {
  const { themes }: { themes: Theme[] } = get(states);
  return themes;
};

export default {
  states,
  initThemesStore,
  resetCurrentTheme,
  updateThemesState,
  updateCurrentThemeState,
  getThemeById,
  getCurrentThemeState,
  getThemesState,
};
