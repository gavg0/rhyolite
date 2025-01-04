<script lang="ts" module>
    import { invoke } from "@tauri-apps/api/core";
    import Workspace from "../../routes/workspace.svelte";
    import type { Context } from "quill/modules/keyboard";
    import { setContext, getContext, onMount } from "svelte";
    import {
            updateCurrentID,
            getCurrentID,
            toggleCommandPalette
        }
        from "../../routes/workspace.svelte";
    import {
            updateTitleText,
            returnTitleText
        }
        from "../components/titlebox.svelte";

    

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
    // const editor: any = getContext('editor');

    // setContext(
    //     'tabs',
    //     {
    //         updateTabs,
    //         addnewtab,
    //         switchTab,
    //         getTabs,
    //         cycleTabs,
    //         gotoLastTab,
    //         gotoTab1,
    //         returnTabsArray
    //     }
    // );
    

    export function returnTabsArray(): Tab[] {
        return currentTabs;
    }

    export async function getTabs(): Promise<Tab[]> {
        return await invoke("get_tabs"); // New Rust function needed
    }

    export async function updateTabs(): Promise<void> {
        currentTabs = await getTabs();
    }

    export async function addnewtab(): Promise<void> {
        const newTab: Tab = await invoke("new_tab");
        updateCurrentID(newTab.id);
        updateTitleText(newTab.title);
        editor.setEditorContent('');
        await updateTabs();
        await invoke("send_current_open_tab", { id: newTab.id });
    }

    export async function switchTab(tabId: string): Promise<void> {
        try {
            const docResult: Document | null = await invoke(
                "get_document_content",
                { id: tabId }
            );

            if (docResult) {
                updateCurrentID(tabId);
                updateTitleText(docResult.title);
                editor.setEditorContent(docResult.content);
            } else {
                updateCurrentID(tabId);
                updateTitleText("");
                editor.setEditorContent("");
            }
            
            // Update the current open tab in the backend
            await invoke("send_current_open_tab", { id: tabId });
        } catch (error) {
            console.error("Failed to switch tab:", error);
        }
    }

    export async function cycleTabs(): Promise<void> {
        try {
            const nextTabId: string = await invoke("cycle_tabs");
            const docResult: Document | null = await invoke("get_document_content", { id: nextTabId });
            
            if (docResult) {
                updateCurrentID(nextTabId);
                updateTitleText(docResult.title);
                editor.setEditorContent(docResult.content);
            }
        } catch (error) {
            console.error("Failed to cycle tabs:", error);
        }
    }

    export async function gotoTab1(): Promise<void> {
        const tabs = await getTabs();
        if (tabs.length > 0) {
            await switchTab(tabs[0].id);
        }
    }

    export async function gotoLastTab(): Promise<void> {
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