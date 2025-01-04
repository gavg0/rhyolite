<script lang="ts">
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

    

    
</script>

<!-- <svelte:window on:keydown={handleKeydown} /> -->

<main>
    <Workspace/>
</main>
