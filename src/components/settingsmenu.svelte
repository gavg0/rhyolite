<script lang="ts">
  import { onDestroy, type Component } from "svelte";
  import settingsMenuState from "../stores/settings-menu.store";
  import ThemeStore from "../stores/theme.store";
  import type { Theme } from "../types/theme";

  let settingsVisible = $state(false);
  let buttonPosition = { top: 150, left: 40 };
  let boxDimensions = { width: 200, height: 200 };
  let themes: Theme[] = $state([]);
  let currentTheme: Theme | undefined = undefined;
  let showThemeOptions = $state(false); 

  const unsubscribeThemeStore = ThemeStore.states.subscribe((v) => {
    currentTheme = v.currentTheme;
    themes = v.themes;
  });

  const menuButtons = [
    { label: "General Settings", onClick: () => console.log("Opening General Settings...") },
    { label: "Theme", onClick: () => showThemeOptions = !showThemeOptions }, 
    { label: "Keyboard Shortcuts", onClick: () => console.log("Opening Keyboard Shortcuts...") },
    { label: "About", onClick: () => console.log("Opening About...") },
    { label: "Close", onClick: () => settingsMenuState.toggleSettingsMenu() },
  ];

  const unsubscribeSettingsMenuStore = settingsMenuState.subscribe(state => {
    settingsVisible = state.settingsMenuVisible;
  });

  onDestroy(() => {
    unsubscribeSettingsMenuStore();
    unsubscribeThemeStore();
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
      settingsVisible = false;
      document.removeEventListener("click", closeSettingsOnClickOutside);
    }
  }

  $effect(() => {
    if (self) {
     document.addEventListener("click", closeSettingsOnClickOutside); 
    }
  });
</script>

{#if settingsVisible}
  <div
    bind:this={self}
    class="absolute rounded-lg p-2 z-50 transition-all duration-300 transform bg-base shadow-lg"
    class:translate-y-0={settingsVisible}
    class:opacity-100={settingsVisible}
    class:translate-y-5={!settingsVisible}
    class:opacity-0={!settingsVisible}
    style="top: {buttonPosition.top}px; left: {buttonPosition.left}px; width: {boxDimensions.width}px;"

  >
    {#each menuButtons as { label, onClick }}
      <button
        class="w-full p-2 rounded text-left text-text bg-transparent cursor-pointer transition-all duration-300 text-sm hover:bg-surface1 focus:bg-surface1"
        onclick={onClick}
      >
        {label}
      </button>
    {/each}

    {#if showThemeOptions}
    <div class="absolute left-full rounded-lg p-4 top-0 mt-4 p-2 w-max bg-base shadow-lg"
    style="width: {boxDimensions.width}px;"
    >
        {#each themes as theme}
          <button
            class="w-full p-2 text-left text-text bg-transparent cursor-pointer transition-all duration-300 text-sm hover:bg-surface1 focus:bg-surface1"
            onclick={() => changeTheme(theme)}
          >
            {theme.name}
          </button>
        {/each}
      </div>
    {/if}
  </div>
{/if}
