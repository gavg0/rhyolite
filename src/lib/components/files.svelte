<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount, getContext, setContext } from "svelte";
    import Close from "$lib/static/close.svg";

    interface DocumentData {
        id: string;
        title: string;
        content: string;
    }

    interface RecentFileInfo {
        id: string;
        title: string;
    }

    let files: RecentFileInfo[] = $state([]);
    let selectedIndex: number = $state(-1);

    const editor: any = getContext("editor");

    onMount(() => {
        loadFiles(); // Load files when component mounts
    });

    async function loadFiles() {
        try {
            // Now we only load the metadata
            files = await invoke("get_recent_files_metadata");
        } catch (error) {
            console.error("Failed to load files:", error);
        }
    }

    async function openFile(doc: RecentFileInfo): Promise<void> {
        // Only load the full content when user clicks
        const fullDoc: DocumentData = await invoke("get_document_content", {
            id: doc.id,
        });
        if (fullDoc) {
            await editor.addnewtab();
            editor.setEditorContent(fullDoc.content);
            editor.setTitleText(doc.title);
            toggleFiles();
        }
    }

    function toggleFiles(): void {
        editor.toggleFilesMenu();
        if (editor.return_isFilesMenuvisible()) {
            loadFiles();
        }
    }

    function handleKeydown(event: KeyboardEvent) {
        if (!editor.return_isFilesMenuvisible()) return;

        switch (event.key) {
            case "ArrowDown":
                event.preventDefault();
                selectedIndex = (selectedIndex + 1) % files.length;
                break;
            case "ArrowUp":
                event.preventDefault();
                selectedIndex =
                    (selectedIndex - 1 + files.length) % files.length;
                break;
            case "Enter":
                event.preventDefault();
                if (selectedIndex >= 0 && selectedIndex < files.length) {
                    openFile(files[selectedIndex]);
                }
                break;
            case "Escape":
                event.preventDefault();
                toggleFiles();
                break;
        }
    }

    // Reset selected index when files view is closed
    $effect(() => {
        if (!editor.return_isFilesMenuvisible()) {
            selectedIndex = -1;
        }
    });
    $effect(() => {
        if (editor.return_isFilesMenuvisible()) {
            loadFiles();
        }
    });
</script>

<svelte:window on:keydown={handleKeydown} />

<main>
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    {#if editor.return_isFilesMenuvisible()}
        <!-- Background overlay -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <div
            class="fixed top-0 left-0 w-full h-full bg-black/40 z-50"
            onclick={() => toggleFiles()}
        ></div>

        <!-- Files list container -->
        <div
            class="fixed top-[40%] left-1/2 flex flex-col bg-crust border-2 border-subtext0 rounded-lg p-3 z-[60] min-w-[400px] max-w-[800px] max-h-[400px] w-[80%] h-[60%] gap-2 -translate-x-1/2 -translate-y-1/2 overflow-y-scroll"
        >
            <div
                class="relative w-full max-h-[40px] h-[16%] mb-2 flex justify-between items-center"
            >
                <span class="text-text text-lg">Recent Files</span>
                <button
                    class="bg-transparent text-text opacity-70 hover:bg-surface0 rounded-full p-2 transition-opacity duration-200"
                    onclick={() => toggleFiles()}
                >
                    <img src={Close} alt="close" />
                </button>
            </div>

            {#each files as file, index}
                <button
                    type="button"
                    class={`flex justify-between items-center px-2 py-1 hover:bg-surface0 cursor-pointer w-full h-[14%] text-left text-text border-none shadow-none rounded transition-colors duration-200 ${selectedIndex === index ? "bg-surface0" : "bg-transparent"}`}
                    onclick={() => openFile(file)}
                    onmouseenter={() => (selectedIndex = index)}
                >
                    <span>{file.title || "Untitled"}</span>
                </button>
            {/each}
        </div>
    {/if}
</main>
