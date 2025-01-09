import type { Tab } from "../types/tab";
import TabsStore, {type ITabsStates} from "../stores/tabs.store";
import { ApiProvider } from "./api.service";

const apiProvider = new ApiProvider();

const switchTab = async (tabId: string): Promise<Tab | undefined> => {
    const tab: Tab | undefined = TabsStore.getTabById(tabId);
    if(!tab) return undefined;

    TabsStore.updateCurrentTabState(tab ?? null);

    await apiProvider.sendCurrentOpenTab(tab.id);

    return tab;
};


const gotoTab1 = async () => {
    const tabs: Tab[] = TabsStore.getTabsState();
    if (tabs.length > 0) {
        await switchTab(tabs[0].id);
    }
};

const gotoLastTab = async ()=> {
    const tabs: Tab[] = TabsStore.getTabsState();
    if (tabs.length > 0) {
        const lastTabIndex = tabs.length - 1;
        await switchTab(tabs[lastTabIndex].id);
    }
};

const cycleTabs = async () => {
    const tabs: Tab[] = TabsStore.getTabsState();
    const currentTab: Tab | null = TabsStore.getCurrentTabState();
    if (tabs.length > 0) {
        const currentTabIndex = tabs.findIndex(tab => tab.id === currentTab?.id);
        const nextTabIndex = (currentTabIndex + 1) % tabs.length;
        const nextTab = tabs[nextTabIndex];
        await switchTab(nextTab.id);
    }
}

const updateTabTitleById = async (tabId: string, newTitle: string) => {
    TabsStore.states.update((data: ITabsStates) => {
        return {
            ...data,
            tabs: data.tabs.map(tab =>
                tab.id === tabId ? { ...tab, title: newTitle } : tab
            )
        };
    });
}

export default {
    switchTab,
    gotoTab1,
    gotoLastTab,
    cycleTabs,
    updateTabTitleById,
}