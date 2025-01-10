<script lang="ts">
    import { Tabs, } from 'flowbite-svelte';
    import DocumentTabItem from "./document-tab-item.svelte";
    import { onDestroy } from 'svelte';
    import TabsStore from "../stores/tabs.store";
    import {type Tab, TabType} from "../types/tab";

    let tabs: Tab[] = $state([]);
    let currentTab: Tab | null = $state(null);

    const unsubscribeTabsState = TabsStore.states.subscribe(value => {
        tabs = value.tabs;
        currentTab = value.currentTab;
    });
    onDestroy(unsubscribeTabsState); // Clean up

    const onOpenTab = (tab: Tab) => {
        TabsStore.updateCurrentTabState(tab);
    }
</script>

<Tabs>
    {#each tabs as tab}
        {#if tab.tabType === TabType.Document || tab.tabType === undefined}
            <DocumentTabItem {tab} open={ currentTab?.id === tab.id } onclick={() => onOpenTab(tab)}/>
        {/if}
    {/each}
</Tabs>