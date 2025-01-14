<script lang="ts">
  import "../styles/styles.css";

  import { onMount } from "svelte";
  import CommandPalette from "../components/command-palette.svelte";

  import Workspace from "../components/workspace.svelte";
  import TitleBar from "../components/titlebar.svelte";
  import HomeHotkeys from "../components/home-hotkeys.svelte";
  import DocumentService from "../services/document.service";
  import Sidebar from "../components/sidebar.svelte";

  onMount(() => {
    // TabsStore.initTabsStore();
    DocumentService.loadRecentDocuments();
  });
</script>

<main class="flex flex-col h-lvh">
  <TitleBar />
  <div class="flex items-stretch grow overflow-hidden">
    <Sidebar />
    <Workspace />
  </div>
  <HomeHotkeys />
  <CommandPalette />
</main>
