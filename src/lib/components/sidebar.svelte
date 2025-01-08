<script lang="ts">
    import { setContext, getContext } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import CommandIcon from "$lib/static/command_palette.svg"
    import RecentFilesIcon from "$lib/static/recent_files.svg"

    const editor: any = getContext('editor');

</script>

<main class="fixed left-0 top-0 h-full w-[35px] border-r-2 border-r-surface0">
    <div class="flex flex-col mt-[40px] py-[10px] px-[2px]">
        <button class="flex justify-center rounded-lg items-center h-5 px-0.5 py-4 w-full cursor-pointer hover:bg-text/10 transition-all duration-200focus:outline-none focus:ring-0" 
        id="CommandPalette_button" 
        onclick={() => editor.toggleCommandPalette()} 
        aria-label="Open Command Palette"
        >
            <img
                src={ CommandIcon }
                alt="Open Command Palette"
                class=""
            />
        </button>
        <button class="flex justify-center rounded-lg items-center h-5 px-0.5 py-4 w-full cursor-pointer hover:bg-text/10 transition-all duration-200focus:outline-none focus:ring-0" 
        id="CommandPalette_button" 
        onclick={() => editor.toggleFilesMenu()} 
        aria-label="Open Command Palette"
        >
            <img
                src={ RecentFilesIcon }
                alt="Open Command Palette"
                class=""
            />
        </button>
    </div>
</main>