<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { setContext, getContext } from "svelte";
    import Workspace from "../../routes/workspace.svelte";
    import type { Context } from "quill/modules/keyboard";

    let titleText: string = $state("");
    setContext(
        'title',
        {
            updateTitleText,
            returnTitleText
        }
    );
    function updateTitleText(Text: string) {
        titleText = Text;
    }
    function returnTitleText(): string {
        return titleText;
    }

    function handleTitleChange(event: Event): void {
        const target = event.target as HTMLTextAreaElement;
        titleText = target.value;
    }
</script>

<main>
    <div class="flex w-[60%] h-[10%]">
        <textarea
            class="w-full h-full"
            placeholder="Enter Title here..."
            value={titleText}
            oninput={handleTitleChange}
        ></textarea>
    </div>
</main>
