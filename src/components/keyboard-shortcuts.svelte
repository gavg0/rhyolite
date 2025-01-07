<script lang="ts">
    import DocumentService from "../services/document.service";
    import TabService from "../services/tab.service";

    const handleKeydown = (event: KeyboardEvent): void => {
        if (event.ctrlKey && event.key === "d") {
            event.preventDefault();
            DocumentService.deleteDocumentTab();
        }
        if (event.ctrlKey && event.key === "n") {
            event.preventDefault();
            DocumentService.addNewDocumentTab();
        }
        if (event.ctrlKey && event.key === "t") {
            event.preventDefault();
            // toggleToolbar();
        }
        if (
            (event.ctrlKey && event.key === "Tab") ||
            (event.ctrlKey && event.key === "PageDown")
        ) {
            event.preventDefault();
            TabService.cycleTabs();
        }
        if (event.ctrlKey && event.key === "1") {
            event.preventDefault();
            TabService.gotoTab1();
        }
        if (event.ctrlKey && event.key === "9") {
            event.preventDefault();
            TabService.gotoLastTab();
        }
        if (event.ctrlKey && event.key === "p") {
            event.preventDefault();
            // toggleCommandPalette();
        }
    }
</script>

<svelte:window on:keydown={handleKeydown} />