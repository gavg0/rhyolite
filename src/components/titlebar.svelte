<script lang="ts">
  import Close from "$lib/static/close.svg";
  import Maximise from "$lib/static/maximise.svg";
  import Minimise from "$lib/static/minimise.svg";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onDestroy } from "svelte";
  import TabsStore from "../stores/tabs.store";
  import { type Tab } from "../types/tab";
  import { addNewDocumentTab } from "../services/document.service";
  import ThemeStore from "../stores/theme.store";
  import type { Theme } from "../types/theme";
  import { Select } from "flowbite-svelte";

  let tabs: Tab[] = $state([]);
  let currentTab: Tab | null = $state(null);

  const unsubscribeTabsState = TabsStore.states.subscribe((value) => {
    tabs = value.tabs;
    currentTab = value.currentTab;
  });
  let themes: Theme[] = $state([]);
  let currentTheme: Theme | undefined = $state();
  const unsubscribeThemeStore = ThemeStore.states.subscribe((v) => {
    currentTheme = v.currentTheme;
    themes = v.themes;
  });
  onDestroy(() => {
    unsubscribeTabsState();
    unsubscribeThemeStore();
  }); // Clean up
  $effect(() => {
    if (currentTab) {
      document
        .querySelector(
          currentTab.id === tabs[tabs.length - 1].id
            ? "#tablist>#new-tab-btn"
            : "#tablist>.active",
        )
        ?.scrollIntoView({
          behavior: "smooth",
          block: "nearest",
          inline: "nearest",
        });
    }
  });

  const appWindow = getCurrentWindow();
  const onOpenTab = (tab: Tab) => {
    TabsStore.updateCurrentTabState(tab);
  };
  const selectOptions = $derived(
    themes.map((t) => ({ value: t.name, name: t.name })),
  );
  let curThemeName = $derived(currentTheme?.name);
</script>

<div
  data-tauri-drag-region
  class="flex grow-0 shrink-0 bg-base w-full basis-[40px] select-none justify-between items-center px-1 overflow-hidden"
>
  <div
    class="flex items-center h-full px-4 flex-shrink-1 flex-grow-0 overflow-y-hidden overflow-x-auto gap-1"
    role="tablist"
    id="tablist"
    aria-label="Document tabs"
    data-tauri-drag-region
  >
    {#each tabs as tab}
      <button
        class={`flex justify-left items-center px-4 text-nowrap h-[30px] min-w-[120px] rounded-[18px] flex-shrink text-text hover:bg-surface1 ${currentTab?.id === tab.id ? "bg-surface0" : ""}`}
        class:active={currentTab?.id === tab.id}
        role="tab"
        aria-controls="editor"
        onclick={() => onOpenTab(tab)}
      >
        {tab.title.length > 20
          ? tab.title.slice(0, 20) + "..."
          : tab.title || "Untitled"}
      </button>
    {/each}

    <button
      type="button"
      class="flex justify-center items-center px-4 text-nowrap h-[30px] w-[30px] aspect-square rounded-[18px] flex-shrink text-text hover:bg-surface1"
      id="new-tab-btn"
      onclick={addNewDocumentTab}>+</button
    >
  </div>
  <div class="flex-grow"></div>
  <div class="flex-shrink-0">
    <Select
      size="sm"
      defaultClass="rounded-lg"
      placeholder="Select Theme"
      items={selectOptions}
      value={curThemeName}
      on:change={(e) => {
        ThemeStore.updateCurrentThemeState(
          themes.find(
            (t) => t.name === (e.currentTarget as HTMLSelectElement).value,
          ) as Theme,
        );
      }}
      on:input={(e) => console.log(e)}
    ></Select>
  </div>
  <div class="flex flex-row items-stretch self-stretch flex-shrink-0">
    <button
      class="px-3 cursor-pointer focus-visible:bg-surface2 hover:bg-surface2"
      id="titlebar-minimize"
      onclick={() => appWindow.minimize()}
      aria-label="Minimize"
    >
      <img src={Minimise} alt="minimize" />
    </button>
    <button
      class="px-3 cursor-pointer focus-visible:bg-surface2 hover:bg-surface2"
      id="titlebar-maximize"
      onclick={() => appWindow.toggleMaximize()}
      aria-label="Maximise"
    >
      <img src={Maximise} alt="maximize" />
    </button>
    <button
      class="px-3 cursor-pointer focus-visible:bg-red-700 hover:bg-red-700"
      id="titlebar-close"
      onclick={() => appWindow.close()}
      aria-label="Close"
    >
      <img src={Close} alt="close" />
    </button>
  </div>
</div>
