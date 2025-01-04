<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import Workspace from "../../routes/workspace.svelte";
    import type { Context } from "quill/modules/keyboard";
    import { setContext, getContext } from "svelte";

    

    interface Tab {
        id: string;
        title: string;
    }

    interface Document {
        id: string;
        title: string;
        content: string;
    }

    let currentTabs: Tab[] = $state([]);

    setContext(
        'tabs',
        {
            updateTabs,
            addnewtab,
            switchTab,
            getTabs,
            cycleTabs,
            gotoLastTab,
            gotoTab1,
            returnTabsArray
        }
    );
    
    const workspace: any = getContext('workspace');
    const title: any = getContext('Title');
    const editor: any = getContext('editor');

    function returnTabsArray(): Tab[] {
        return currentTabs;
    }

    async function getTabs(): Promise<Tab[]> {
        return await invoke("get_tabs"); // New Rust function needed
    }

    async function updateTabs(): Promise<void> {
        currentTabs = await getTabs();
    }

    async function addnewtab(): Promise<void> {
        const newTab: Tab = await invoke("new_tab");
        workspace.updateCurrentID(newTab.id);
        title.updateTitleText(newTab.title);
        editor.setEditorContent('');
        await updateTabs();
        await invoke("send_current_open_tab", { id: newTab.id });
    }

    async function switchTab(tabId: string): Promise<void> {
        try {
            const docResult: Document | null = await invoke(
                "get_document_content",
                { id: tabId }
            );

            if (docResult) {
                workspace.updateCurrentID(tabId);
                title.updateTitleText(docResult.title);
                editor.setEditorContent(docResult.content);
            } else {
                workspace.updateCurrentID(tabId);
                title.updateTitleText("");
                editor.setEditorContent("");
            }
            
            // Update the current open tab in the backend
            await invoke("send_current_open_tab", { id: tabId });
        } catch (error) {
            console.error("Failed to switch tab:", error);
        }
    }

    async function cycleTabs(): Promise<void> {
        try {
            const nextTabId: string = await invoke("cycle_tabs");
            const docResult: Document | null = await invoke("get_document_content", { id: nextTabId });
            
            if (docResult) {
                workspace.updateCurrentID(nextTabId);
                title.updateTitleText(docResult.title);
                editor.setEditorContent(docResult.content);
            }
        } catch (error) {
            console.error("Failed to cycle tabs:", error);
        }
    }

    async function gotoTab1(): Promise<void> {
        const tabs = await getTabs();
        if (tabs.length > 0) {
            await switchTab(tabs[0].id);
        }
    }

    async function gotoLastTab(): Promise<void> {
        const tabs = await getTabs();
        if (tabs.length > 0) {
            await switchTab(tabs[tabs.length - 1].id);
        }
    }

</script>

<div class="fixed bg-base top-[0px] w-full h-[5%]" role="tablist" aria-label="Document tabs">
    {#each currentTabs as tab}
        <button
            type="button"
            class="flex justify-left h-[30px] w-auto flex-shrink bg-crust"
            
            role="tab"
            
            aria-controls="editor"
            onclick={() => switchTab(tab.id)}
        >
            {tab.title.length > 10 ? tab.title.slice(0, 20) + '...' : tab.title || 'Untitled'}
        </button>
    {/each}
</div>