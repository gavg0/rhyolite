<script lang="ts">
    import { setContext, getContext } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import CommandIcon from "$lib/static/command_palette.svg.svelte";
    import RecentFilesIcon from "$lib/static/recent_files.svg.svelte";

    const editor: any = getContext('editor');

</script>

<main class="fixed left-0 top-0 h-full w-[35px] border-r-2 border-r-surface0">
    <div class="flex flex-col mt-[40px] py-[10px] px-[2px] gap-2">
        <button class="flex justify-center rounded-lg items-center h-5 px-0.5 py-4 w-full cursor-pointer hover:bg-text/10 transition-all duration-200 focus:outline-none focus:ring-0" 
        id="CommandPalette_button" 
        onclick={() => editor.toggleCommandPalette()} 
        aria-label="Open Command Palette"
        title="Open Command Palette"
        >
            <CommandIcon/>
        </button>
        <button class="flex justify-center rounded-lg items-center h-5 px-0.5 py-4 w-full cursor-pointer hover:bg-text/10 transition-all duration-200 focus:outline-none focus:ring-0" 
        id="Files_Menu_button" 
        onclick={() => editor.toggleFilesMenu()} 
        aria-label="Open Files Menu"
        title="Open Files Menu"
        >
            <RecentFilesIcon/>
        </button>
    </div>
</main>