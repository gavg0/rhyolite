<script lang="ts">
    import "../styles/styles.css";

    import { onMount } from "svelte";
    import CommandPalette from "../components/command-palette.svelte";

    import Tabs from "../components/tabs.svelte";
    import HomeHotkeys from "../components/home-hotkeys.svelte";
    import DocumentService from "../services/document.service";

    onMount(() => {
        // TabsStore.initTabsStore();
        DocumentService.loadRecentDocuments();
    });

    // function countWords(text: string): number {
    //     return text.split(/\s+/).filter((word) => word.length > 0).length;
    // }
</script>

<main>
    <HomeHotkeys />

    <div class="p-5">
        <Tabs />
    </div>

    <CommandPalette />
</main>

