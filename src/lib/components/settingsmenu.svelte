<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import settingsMenuState from "../../stores/settings-menu.store";
  import { writable } from "svelte/store";
  import ThemeStore from "../../stores/theme.store";
    import type { Theme } from "@tauri-apps/api/window";

  let settingsVisible = false;
  let AppearanceVisible = false;
  // Button position info (top and left)
  let buttonPosition = { top: 150, left: 40 };

  // List of buttons for the settings menu
  const menuButtons = [
    { label: "General Settings", onClick: () => console.log("Opening General Settings...") },
    { label: "Appearance", onClick: () => console.log("Opening Appearance...") },
    { label: "Keyboard Shortcuts", onClick: () => console.log("Opening Keyboard Shortcuts...") },
    { label: "About", onClick: () => console.log("Opening About...") },
    { label: "Close", onClick: () => settingsMenuState.toggleSettingsMenu() },
  ];
  // let themes: Theme[] = $state([]);
  // let currentTheme: Theme | undefined = $state();
  // const unsubscribeThemeStore = ThemeStore.states.subscribe((v) => {
  //   currentTheme = v.currentTheme as unknown as Theme;
  //   themes = v.themes as unknown as Theme[];
  // });

  const unsubscribeSettingsMenuStore = settingsMenuState.subscibe(state => {
    settingsVisible = state.settingsMenuVisible;
  });

  onDestroy(() => {
    unsubscribeSettingsMenuStore();
    // unsubscribeThemeStore();
  });

</script>

<style>
  .settings-menu {
    position: absolute;
    background-color: #2e2e2e;
    border-radius: 8px;
    padding: 16px;
    z-index: 1000;
    transition: transform 0.3s ease;
    transform: translateY(20px); /* Initially positioned just below the button */
    opacity: 0; /* Start as invisible */
    min-width: 75px;  /* Set a minimum width */
    max-width: 200px;  /* Optional: Max width for the menu */
    width: auto;  /* Make width auto so it adapts to content size */
  }

  .settings-menu.open {
    transform: translateY(0); /* Slide up from below */
    opacity: 1; /* Fade in */
  }

  .settings-menu button {
    width: 100%;
    padding: 10px;
    text-align: left;
    background: transparent;
    border: none;
    color: #ffffff;
    cursor: pointer;
    transition: background 0.3s;
    font-size: 12px;
  }

  .settings-menu button:hover {
    background-color: rgba(255, 255, 255, 0.1);
  }

  .settings-menu button:focus {
    outline: none;
    background-color: rgba(255, 255, 255, 0.2);
  }
</style>



<!-- Settings Menu -->
{#if settingsVisible}
  <div
    class="settings-menu open"
    style="top: {buttonPosition.top}px; left: {buttonPosition.left}px"
  >
    {#each menuButtons as { label, onClick }}
      <button on:click={onClick}>{label}</button>
    {/each}
  </div>
{/if}
