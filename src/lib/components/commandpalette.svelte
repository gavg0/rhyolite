<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { getContext } from "svelte";

    let selectedindex: number = $state(-1);
    let searchText: string = $state("");

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
        action: () => {
            editor.deleteDocument();
            editor.toggleCommandPalette();
        }
      },
      {
        name: 'New Tab',
        shortcut: 'Ctrl + N',
        action: () => {
            editor.newDocument();
            editor.toggleCommandPalette();
        }
      },
      {
        name: 'Next Tab',
        shortcut: 'Ctrl + Tab or Ctrl + pgDown',
        action: () => {
            editor.cycleTabs();
            editor.toggleCommandPalette();
        }
      },
      {
        name: 'Go to First Tab',
        shortcut: 'Ctrl + 1',
        action: () => {
            editor.gotoTab1();
            editor.toggleCommandPalette();
        }
      },
      {
        name: 'Go to Last Tab',
        shortcut: 'Ctrl + 9',
        action: () => {
            editor.gotoLastTab();
            editor.toggleCommandPalette();
        }
      },
      {
        name: 'Close Tab',
        shortcut: 'Ctrl + Alt + C',
        action: () => {
            editor.closeTab();
            editor.toggleCommandPalette();
        }
      }
    //   {
    //     name: 'Toggle ToolBar',
    //     shortcut: 'Ctrl + T',
    //     action: () => {
    //         editor.toggleToolbar();
    //         editor.toggleCommandPalette();
    //     }
    //   }
    ];

    function handleKeydown(event: KeyboardEvent) {
        if (!editor.return_isCommandPalettevisible()) return;

        switch (event.key) {
            case 'ArrowDown':
                event.preventDefault();
                if (selectedindex === -1) {
                    selectedindex = 0;
                } else {
                    selectedindex = (selectedindex + 1) % commands.length;
                }
                break;
            case 'ArrowUp':
                event.preventDefault();
                if (selectedindex === -1) {
                    selectedindex = commands.length - 1;
                } else {
                    selectedindex = (selectedindex - 1 + commands.length) % commands.length;
                }
                break;
            case 'Enter':
                event.preventDefault();
                if (selectedindex >= 0 && selectedindex < commands.length) {
                    commands[selectedindex].action();
                }
                break;
            case 'Escape':
                event.preventDefault();
                editor.toggleCommandPalette();
                break;
        }
    }

    // function handleWheel(event: WheelEvent) {
    //     if (!editor.return_isCommandPalettevisible()) return;
        
    //     event.preventDefault();
    //     if (event.deltaY > 0) {
    //         // Scrolling down
    //         selectedindex = (selectedindex + 1) % commands.length;
    //     } else {
    //         // Scrolling up
    //         selectedindex = (selectedindex - 1 + commands.length) % commands.length;
    //     }
    // }

    // Reset selected index when command palette is closed
    $effect(() => {
        if (!editor.return_isCommandPalettevisible()) {
            selectedindex = -1;
            searchText = "";
        }
    });
</script>

<svelte:window on:keydown={handleKeydown} />

<main>
    {#if editor.return_isCommandPalettevisible()}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div 
            class="fixed top-0 left-0 w-full h-full bg-black/60 z-50"
            onclick={() => editor.toggleCommandPalette()}
        ></div>
        <div 
            class="fixed top-[40%] left-1/2 flex flex-col bg-crust border-2 border-subtext0 rounded-lg p-3 z-[60] min-w-[400px] max-w-[800px] max-h-[400px] w-[80%] h-[60%] gap-2 -translate-x-1/2 -translate-y-1/2 overflow-y-scroll"
        >
            <div class="relative w-full max-h-[40px] h-[16%] mb-2">
                <textarea
                    class="flex w-full h-full overflow-hidden resize-none pr-8 p-2 cursor-text text-text bg-surface0 text-left box-border border-2 border-subtext0 outline-none rounded transition-all duration-200 hover:border-overlay0 focus:border-overlay0 focus:outline-none focus:ring-0"
                    placeholder="Select a Command"
                    bind:value={searchText}
                ></textarea>
                <button 
                    class="absolute right-2 top-1/2 -translate-y-1/2 bg-transparent text-text opacity-70 hover:opacity-100 transition-opacity duration-200"
                    onclick={() => editor.toggleCommandPalette()}
                >
                    ✕
                </button>
            </div>
            {#each commands as command, index}
                <button
                    type="button"
                    class={`flex justify-between items-center px-2 py-1 hover:bg-surface0 cursor-pointer w-full h-[14%] text-left text-text border-none shadow-none rounded transition-colors duration-200 ${selectedindex === index ? 'bg-surface0' : 'bg-transparent'}`}
                    onclick={() => command.action()}
                    onmouseenter={() => selectedindex = index}
                >
                    <span>{command.name}</span>
                    <span class="opacity-100 text-subtext0">{command.shortcut}</span>
                </button>
            {/each}
        </div>
    {/if}
</main>