<script lang="ts">
    import CommandPaletteStore from "../store/command-palette.store";
    import DocumentService from "../services/document.service";
    import TabService from "../services/tab.service";
    import { onDestroy } from "svelte";
    import ContentEditorStore from "../store/content-editor.store";

    let selectedIndex: number = $state(-1);
    let searchText: string = $state("");

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
            DocumentService.deleteDocumentTab();
            CommandPaletteStore.toggleVisibility();
        }
      },
      {
        name: 'New Tab',
        shortcut: 'Ctrl + N',
        action: () => {
            DocumentService.addNewDocumentTab();
            CommandPaletteStore.toggleVisibility();
        }
      },
      {
        name: 'Next Tab',
        shortcut: 'Ctrl + Tab or Ctrl + pgDown',
        action: () => {
            TabService.cycleTabs();
            CommandPaletteStore.toggleVisibility();
        }
      },
      {
        name: 'Go to First Tab',
        shortcut: 'Ctrl + 1',
        action: () => {
            TabService.gotoTab1();
            CommandPaletteStore.toggleVisibility();
        }
      },
      {
        name: 'Go to Last Tab',
        shortcut: 'Ctrl + 9',
        action: () => {
            TabService.gotoLastTab();
            CommandPaletteStore.toggleVisibility();
        }
      },
      {
        name: 'Toggle ToolBar',
        shortcut: 'Ctrl + T',
        action: () => {
            ContentEditorStore.toggleToolbarVisibility();
            CommandPaletteStore.toggleVisibility();
        }
      }
    ];

    function handleKeydown(event: KeyboardEvent) {
        if (!CommandPaletteStore.isVisible()) return;

        switch (event.key) {
            case 'ArrowDown':
                event.preventDefault();
                if (selectedIndex === -1) {
                    selectedIndex = 0;
                } else {
                    selectedIndex = (selectedIndex + 1) % commands.length;
                }
                break;
            case 'ArrowUp':
                event.preventDefault();
                if (selectedIndex === -1) {
                    selectedIndex = commands.length - 1;
                } else {
                    selectedIndex = (selectedIndex - 1 + commands.length) % commands.length;
                }
                break;
            case 'Enter':
                event.preventDefault();
                if (selectedIndex >= 0 && selectedIndex < commands.length) {
                    commands[selectedIndex].action();
                }
                break;
            case 'Escape':
                event.preventDefault();
                CommandPaletteStore.toggleVisibility();
                break;
        }
    }

    function handleWheel(event: WheelEvent) {
        if (!CommandPaletteStore.isVisible()) return;
        
        event.preventDefault();
        if (event.deltaY > 0) {
            // Scrolling down
            selectedIndex = (selectedIndex + 1) % commands.length;
        } else {
            // Scrolling up
            selectedIndex = (selectedIndex - 1 + commands.length) % commands.length;
        }
    }

    // Reset selected index when command palette is closed
    $effect(() => {
        if (!CommandPaletteStore.isVisible()) {
            selectedIndex = -1;
            searchText = "";
        }
    });

    let flagVisibility = $state(false);
    const unsubscribeStates = CommandPaletteStore.commandPaletteStore.subscribe(value => {
        flagVisibility = value.flagCommandPaletteVisibility;
    });
    onDestroy(unsubscribeStates); // Clean up
</script>

<svelte:window on:keydown={handleKeydown} />

<main>
    {#if flagVisibility}
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <div
            class="background-blur"
            onclick={() => CommandPaletteStore.toggleVisibility()}
            >
        </div>
        <div class="commandPalette" onwheel={handleWheel}>
            <div class="search-container">
                <textarea
                    class="command-search"
                    placeholder="Select a Command"
                    bind:value={searchText}
                ></textarea>
                <button
                    class="close-button"
                    onclick={() => CommandPaletteStore.toggleVisibility()}
                >
                    ✕
                </button>
            </div>
            {#each commands as command, index}
                <button
                    type="button"
                    class="command-item"
                    class:active={selectedIndex === index}
                    onclick={() => {
                        command.action();
                    }}
                    onmouseenter={() => selectedIndex = index}
                >
                    <span>{command.name}</span>
                    <span class="shortcut">{command.shortcut}</span>
                </button>
            {/each}
        </div>
    {/if}
</main>
