<script lang="ts">
    import { setContext, getContext } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    
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

    setContext(
        'io',
        {
            autoSave,
            loadRecentDocuments,
            handleKeydown
        }
    );

    const editor: any = getContext('editor');
    const workspace: any = getContext('workspace');
    const tabs: any = getContext('tabs');
    const title: any = getContext('title');

    async function autoSave(): Promise<void> {
        let titleText = title.returnTitleText();
        if (!titleText && !editor.getEditorContentasText()) return;
        let currentId = workspace.getCurrentID();

        try {
            await invoke("update_tab_title", {
                id: currentId,
                title: titleText,
            });
            await tabs.updateTabs();
            await invoke("save_document", {
                id: currentId,
                title: titleText,
                content: editor.getEditorContent(),
            });
        } catch (error) {
            console.error("Auto-save failed:", error);
        }
    }

    async function loadRecentDocuments(): Promise<void> {
        try {
            const docs: Document[] = await invoke("load_recent_files");
            recentDocuments = docs;

            // Update the tabs in UI
            await tabs.updateTabs();

            if (recentDocuments.length > 0) {
                // Load the last open tab from the backend
                const openTabId: string = await invoke("get_current_open_tab");
                await tabs.switchTab(openTabId);
            } else {
                // If no documents exist, create a new tab
                await tabs.addnewtab();
            }
        } catch (error) {
            console.error("Failed to load documents:", error);
        }
    }

    async function deleteDocument(): Promise<void> {
        try {
            currentId = workspace.getCurrentID();
            // The Rust function returns the next document's content after deletion
            const nextDoc: Document | null = await invoke("delete_document", { id: currentId });
            await tabs.updateTabs();
            
            if (nextDoc) {
                // If we have a next document, switch to it
                workspace.updateCurrentID(nextDoc.id);
                title.updateTitleText(nextDoc.title);
                editor.setEditorContent(nextDoc.content);
            } else {
                // If no documents left, create a new one
                await tabs.addnewtab();
            }
            
            currentId = workspace.getCurrentID();
            await invoke("send_current_open_tab", { id: currentId });
        } catch (error) {
            console.error("Failed to delete document:", error);
        }
    }

    async function newDocument(): Promise<void> {
        try {
            const newTab: Tab = await invoke("new_tab");
            workspace.updateCurrentID(newTab.id);
            title.updateTitleText(newTab.title);
            editor.setEditorContent("");
            await tabs.updateTabs();
        } catch (error) {
            console.error("Failed to create new document:", error);
        }
    }

    function handleKeydown(event: KeyboardEvent): void {
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
            tabs.cycleTabs();
        }
        if (event.ctrlKey && event.key === "1") {
            event.preventDefault();
            tabs.gotoTab1();
        }
        if (event.ctrlKey && event.key === "9") {
            event.preventDefault();
            tabs.gotoLastTab();
        }
        if (event.ctrlKey && event.key === "p") {
            event.preventDefault();
            workspace.toggleCommandPalette();
        }
    }
</script>