<script lang="ts">
  import {
    ChevronRight,
    SlidersHorizontal,
    Palette,
    Keyboard,
    Info,
    X,
  } from "lucide-svelte";
  import { onDestroy, type Component } from "svelte";
  import settingsMenuState from "../stores/settings-menu.store";
  import ThemeStore from "../stores/theme.store";
  import type { Theme } from "../types/theme";
  import settingsMenuStore from "../stores/settings-menu.store";

  let settingsVisible = $state(false);
  let buttonPosition = { top: 150, left: 44, bottom: 15 };
  let boxDimensions = { width: 200, height: 200 };
  let themes: Theme[] = $state([]);
  let currentTheme: Theme | undefined = undefined;
  let showThemeOptions = $state(false);

  const unsubscribeThemeStore = ThemeStore.states.subscribe((v) => {
    currentTheme = v.currentTheme;
    themes = v.themes;
  });

  const menuButtons = [
    {
      label: "General Settings",
      onClick: () => console.log("Opening General Settings..."),
    },
    { label: "Theme", onClick: () => (showThemeOptions = !showThemeOptions) },
    {
      label: "Keyboard Shortcuts",
      onClick: () => console.log("Opening Keyboard Shortcuts..."),
    },
    { label: "About", onClick: () => console.log("Opening About...") },
    { label: "Close", onClick: () => settingsMenuState.toggleSettingsMenu() },
  ];

  const unsubscribeSettingsMenuStore = settingsMenuState.subscribe((state) => {
    settingsVisible = state.settingsMenuVisible;
  });

  onDestroy(() => {
    unsubscribeSettingsMenuStore();
    unsubscribeThemeStore();
    document.removeEventListener("click", closeSettingsOnClickOutside);
  });

  const changeTheme = (theme: Theme) => {
    ThemeStore.updateCurrentThemeState(theme);
    showThemeOptions = false;
    settingsVisible = false;
  };

  let self: HTMLElement | null = $state(null);

  function closeSettingsOnClickOutside(e: MouseEvent) {
    if (!self?.contains(e.target as Node)) {
      e.stopPropagation();
      settingsMenuStore.toggleSettingsMenu();
      document.removeEventListener("click", closeSettingsOnClickOutside);
    }
  }

  function closeSettingsOnEscape(e: KeyboardEvent) {
    if (e.key === "Escape") {
      settingsMenuStore.toggleSettingsMenu();
      document.removeEventListener("keydown", closeSettingsOnEscape);
    }
  }

  $effect(() => {
    if (self) {
      document.addEventListener("click", closeSettingsOnClickOutside);
      document.addEventListener("keydown", closeSettingsOnEscape);
    }
  });
</script>

{#if settingsVisible}
  <div
    bind:this={self}
    class="absolute rounded-lg p-1 pt-[6px] z-50 transition-all duration-300 transform bg-base shadow-xl"
    class:translate-y-0={settingsVisible}
    class:opacity-100={settingsVisible}
    class:translate-y-5={!settingsVisible}
    class:opacity-0={!settingsVisible}
    style="bottom: {buttonPosition.bottom}px; left: {buttonPosition.left}px; width: {boxDimensions.width}px;"
  >
    {#each menuButtons as { label, onClick }}
      <button
        class="w-full p-1 rounded-lg text-left text-text cursor-pointer transition-all duration-300 text-sm hover:bg-surface1 focus:bg-surface1 flex flex-row justify-between items-center"
        onclick={onClick}
      >
        <div class="flex flex-row gap-1.5 items-center">
          {#if label === "Theme"}
            <Palette class="w-4 h-4" />
          {:else if label === "General Settings"}
            <SlidersHorizontal class="w-4 h-4" />
          {:else if label === "Keyboard Shortcuts"}
            <Keyboard class="w-4 h-4" />
          {:else if label === "About"}
            <Info class="w-4 h-4" />
          {:else if label === "Close"}
            <X class="w-4 h-4" />
          {/if}
          {label}
        </div>
        {#if label === "Theme"}
          <ChevronRight class="w-4 h-4" />
        {/if}
      </button>
    {/each}

    {#if showThemeOptions}
      <div
        class="absolute left-full rounded-lg p-1 bottom-[58%] mt-8 ml-1 w-max bg-base shadow-xl"
        style="width: {boxDimensions.width}px;"
      >
        {#each themes as theme}
          <button
            class="w-full p-1 rounded-lg text-left text-text bg-transparent cursor-pointer transition-all duration-300 text-sm hover:bg-surface1 focus:bg-surface1"
            onclick={() => {
              changeTheme(theme), settingsMenuState.toggleSettingsMenu();
            }}
          >
            {theme.name}
          </button>
        {/each}
      </div>
    {/if}
  </div>
{/if}
