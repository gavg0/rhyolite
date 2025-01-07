<script lang="ts">
    import "../styles/styles.css";

    import { onMount } from "svelte";
    // import Commandpalette from "./commandpalette.svelte";

    import Tabs from "../components/tabs.svelte";
    import KeyboardShortcuts from "../components/keyboard-shortcuts.svelte";
    import DocumentService from "../services/document.service";

    onMount(() => {
        // TabsStore.initTabsStore();
        DocumentService.loadRecentDocuments();
    });

    // function countWords(text: string): number {
    //     return text.split(/\s+/).filter((word) => word.length > 0).length;
    // }
</script>

<KeyboardShortcuts />

<div class="p-5">
    <Tabs />
</div>
