<script lang="ts">
     import { invoke } from "@tauri-apps/api/core";
     import { getContext } from 'svelte';
    import Workspace from "../../routes/workspace.svelte";
    import type { Context } from "quill/modules/keyboard";

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
    
    let workspace: any =getContext('workspace');

    async function switchTab(tabId: string): Promise<void> {
        try {
            const docResult: Document | null = await invoke(
                "get_document_content",
                { id: tabId }
            );

            if (docResult) {
                workspace.updateCurrentID(tabId);
                titleText = docResult.title;
                // quill?.setContents(JSON.parse(docResult.content));
            } else {
                workspace.updateCurrentID(tabId);
                titleText = "";
                // quill?.setContents([]);
            }
            
            // Update the current open tab in the backend
            await invoke("send_current_open_tab", { id: tabId });
        } catch (error) {
            console.error("Failed to switch tab:", error);
        }
    }

</script>

<div class="fixed bg-slate-400 top-[0px] w-full h-[5%]" role="tablist" aria-label="Document tabs">
    {#each currentTabs as tab}
        <button
            type="button"
            class="tab-square"
            class:active={currentId === tab.id}
            role="tab"
            aria-selected={currentId === tab.id}
            aria-controls="editor"
            onclick={() => switchTab(tab.id)}
        >
            {tab.title.length > 10 ? tab.title.slice(0, 15) + '...' : tab.title || 'Untitled'}
        </button>
    {/each}
</div>