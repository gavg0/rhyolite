import { type Writable, writable, get } from 'svelte/store';
import type { Tab } from '../types/tab';
import DocumentService from "../services/document.service";

export interface ITabsStore {
    tabs: Tab[];
    currentTab: Tab | null;
}

const tabsStore: Writable<ITabsStore> = writable<ITabsStore>({
    tabs: [],
    currentTab: null,
});

const initTabsStore = async () => {
    const tabs = await DocumentService.getAllDocumentTabs();
    const currentTab = tabs.length > 0 ? tabs[0] : null;
    tabsStore.update( () => ({ tabs, currentTab }));
}

const resetCurrentTab = () => {
    const tabs: Tab[] = getTabsState();
    const currentTab:Tab | null = tabs.length > 0 ? tabs[0] : null;
    updateCurrentTabState(currentTab);
}

const updateTabsState = (tabs: Tab[]): Tab[] => {
    tabsStore.update(data => ({
        tabs: tabs,
        currentTab: data.currentTab,
    }));
    return tabs;
}

const updateCurrentTabState = (currentTab: Tab | null): Tab | null => {
    tabsStore.update(data => ({
        tabs: data.tabs,
        currentTab: currentTab,
    }));
    return currentTab;
}

const getTabById = (tabId: string): Tab | undefined => {
    const { tabs }: { tabs: Tab[] } = get(tabsStore); // Access the current state of tabsStore
    return tabs.find(tab => tab.id === tabId); // Replace 'id' with the actual property name if it's different
};

const getCurrentTabState = (): Tab | null => {
    const { currentTab }: { currentTab: Tab | null } = get(tabsStore);
    return currentTab;
};

const getTabsState = (): Tab[] => {
    const { tabs }: { tabs: Tab[] } = get(tabsStore);
    return tabs;
}

export default {
    tabsStore,
    initTabsStore,
    resetCurrentTab,
    updateTabsState,
    updateCurrentTabState,
    getTabById,
    getCurrentTabState,
    getTabsState,
}
