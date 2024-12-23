<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { getContext } from "svelte";
    import { setContext } from "svelte";

    // let isvisible: boolean = $state(false);
    // let { isvisible: boolean = false} = $props();
    let selectedindex: number = $state(0);

    const editor: any = getContext('editor');

    interface Command {
        name: string;
        shortcut: string;
        action: () => void;
    }

    let commands: Command[] = [
      {
        name: 'Delete Tab',
        shortcut: 'Ctrl + D',
        action: () => editor.deleteDocument()
      },
      {
        name: 'New Tab',
        shortcut: 'Ctrl + N',
        action: () => editor.addnewtab()
      },
      {
        name: 'Next Tab',
        shortcut: 'Ctrl + Tab or Ctrl + pgDown',
        action: () => editor.cycleTabs()
      },
      {
        name: 'Go to First Tab',
        shortcut: 'Ctrl + 1',
        action: () => editor.gotoTab1()
      },
      {
        name: 'Go to Last Tab',
        shortcut: 'Ctrl + 9',
        action: () => editor.gotoLastTab()
      },
      {
        name: 'Toggle ToolBar',
        shortcut: 'Ctrl + T',
        action: () => editor.gotoTab1()
      }
    ];
</script>

<main>
    {#if editor.return_isCommandPalettevisible()}
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <div 
            class="background-blur" 
            onclick={() => editor.toggleCommandPalette()}
            >
        </div>
        <div class="commandPalatte">
            <textarea
                class="command-search"
                placeholder="Select a Command"
                
            ></textarea>
            {#each commands as command, index}
                <button
                    type="button"
                    class="command-item"
                    class:active={selectedindex === index}
                    onclick={() => command.action()}
                >
                    <span>{command.name}</span>
                    <span class="shortcut">{command.shortcut}</span>
                </button>
            {/each}
        </div>
    {/if}
</main>
