<script lang="ts" module>
    import { setContext, getContext } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { get } from 'svelte/store';
    import { editorStore } from '../components/editor';
    import type { Editor } from '@tiptap/core';
    import {
            updateCurrentID,
            getCurrentID,
            toggleCommandPalette
        }
        from "../../routes/workspace.svelte"; 
    import {
            updateTabs,
            addnewtab,
            switchTab,
            getTabs,
            cycleTabs,
            gotoLastTab,
            gotoTab1,
            returnTabsArray
        }
        from "../components/tabsbar.svelte";
    import {
            updateTitleText,
            returnTitleText
        }
        from "../components/titlebox.svelte";

    
    let recentDocuments: Document[] = [];
    let currentId = $state("");

    interface Tab {
        id: string;
        title: string;
    }

    interface Document {
        id: string;
        title: string;
        content: string;
    }

    
    // const editor: any = getContext('editor');
    

    export async function autoSave(): Promise<void> {
        let titleText = returnTitleText();
        const editor: Editor | null = get(editorStore);
        if (!titleText && !editor!.getText()) return;
        let currentId = getCurrentID();

        try {
            await invoke("update_tab_title", {
                id: currentId,
                title: titleText,
            });
            await updateTabs();
            await invoke("save_document", {
                id: currentId,
                title: titleText,
                content: editor!.getHTML(),
            });
        } catch (error) {
            console.error("Auto-save failed:", error);
        }
    }

    export async function loadRecentDocuments(): Promise<void> {
        try {
            const docs: Document[] = await invoke("load_recent_files");
            recentDocuments = docs;

            // Update the tabs in UI
            await updateTabs();

            if (recentDocuments.length > 0) {
                // Load the last open tab from the backend
                const openTabId: string = await invoke("get_current_open_tab");
                await switchTab(openTabId);
            } else {
                // If no documents exist, create a new tab
                await addnewtab();
            }
        } catch (error) {
            console.error("Failed to load documents:", error);
        }
    }

    export async function deleteDocument(): Promise<void> {
        try {
            currentId = getCurrentID();
            // The Rust function returns the next document's content after deletion
            const nextDoc: Document | null = await invoke("delete_document", { id: currentId });
            const editor: Editor | null = get(editorStore);
            await updateTabs();
            
            if (nextDoc) {
                // If we have a next document, switch to it
                updateCurrentID(nextDoc.id);
                updateTitleText(nextDoc.title);
                editor!.commands.setContent(nextDoc.content);
            } else {
                // If no documents left, create a new one
                await addnewtab();
            }
            
            currentId = getCurrentID();
            await invoke("send_current_open_tab", { id: currentId });
        } catch (error) {
            console.error("Failed to delete document:", error);
        }
    }

    export async function newDocument(): Promise<void> {
        try {
            const newTab: Tab = await invoke("new_tab");
            const editor: Editor | null = get(editorStore);
            updateCurrentID(newTab.id);
            updateTitleText(newTab.title);
            editor!.commands.setContent("");
            await updateTabs();
        } catch (error) {
            console.error("Failed to create new document:", error);
        }
    }

    export function handleKeydown(event: KeyboardEvent): void {
        if (event.ctrlKey && event.key === "d") {
            event.preventDefault();
            deleteDocument();
        }
        if (event.ctrlKey && event.key === "n") {
            event.preventDefault();
            newDocument();
        }
        // if (event.ctrlKey && event.key === "t") {
        //     event.preventDefault();
        //     toggleToolbar();
        // }
        if (
            (event.ctrlKey && event.key === "Tab") ||
            (event.ctrlKey && event.key === "PageDown")
        ) {
            event.preventDefault();
            cycleTabs();
        }
        if (event.ctrlKey && event.key === "1") {
            event.preventDefault();
            gotoTab1();
        }
        if (event.ctrlKey && event.key === "9") {
            event.preventDefault();
            gotoLastTab();
        }
        if (event.ctrlKey && event.key === "p") {
            event.preventDefault();
            toggleCommandPalette();
        }
    }
</script>