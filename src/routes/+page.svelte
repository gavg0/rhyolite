<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    import { v4 as uuidv4 } from "uuid";
    import Quill from "quill";
    import { setContext } from 'svelte';
    import Commandpalette from "../lib/components/commandpalette.svelte";
    import Editor from "../lib/components/editor.svelte"
    import Workspace from "../routes/workspace.svelte"

    interface Tab {
        id: string;
        title: string;
    }

    interface Document {
        id: string;
        title: string;
        content: string;
    }

    interface QuickFormatOption {
        icon: string;
        format: string;
        value?: any;
    }

    interface ToolbarOptions {
        [key: string]: any;
    }

    // State management
    let titleText: string = $state("");
    let textboxText: string = "";
    let recentDocuments: Document[] = [];
    let currentId: string = $state("");
    let wordCount: number = $state(0);
    let charCount: number = $state(0);
    let isToolbarVisible: boolean = $state(false);
    let isCommandPalettevisible: boolean = $state(false);
    let currentTabs: Tab[] = $state([]);

    setContext(
        'editor', 
        {   
            return_isCommandPalettevisible: () => isCommandPalettevisible, 
    });

    
</script>

<!-- <svelte:window on:keydown={handleKeydown} /> -->

<main>
    <Workspace/>
</main>
