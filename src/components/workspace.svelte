<script lang="ts">
  import { onDestroy } from "svelte";
  import TabsStore from "../stores/tabs.store";
  import { type Tab, TabType } from "../types/tab";
  import Document from "./document.svelte";

  let tabs: Tab[] = $state([]);
  let currentTab: Tab | null = $state(null);

  const unsubscribeTabsState = TabsStore.states.subscribe((value) => {
    tabs = value.tabs;
    currentTab = value.currentTab;
  });
  onDestroy(unsubscribeTabsState); // Clean up

  const onOpenTab = (tab: Tab) => {
    TabsStore.updateCurrentTabState(tab);
  };
</script>

<div class="flex grow justify-center mt-[30px] px-10 overflow-auto">
  {#each tabs as tab}
    {#if tab.tabType === TabType.Document || tab.tabType === undefined}
      <!-- TODO: Q: How to switch between tabs? -->
      <!-- 1: Have all tabs as separate DOM Elements, set display:none on inactive tabs -->
      <!--    Pro: possibly retained DOM states. Con: Too large DOM-->
      <!-- 2: Have only active tab in DOM -->
      <!--    Pro: possibly retained DOM states. Con: Too large DOM-->
      <Document
        {tab}
        open={currentTab?.id === tab.id}
        onclick={() => onOpenTab(tab)}
      />
    {/if}
  {/each}
</div>
