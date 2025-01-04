<script lang="ts" module>
    import { invoke } from "@tauri-apps/api/core";
    import { setContext, getContext } from "svelte";
    import Workspace from "../../routes/workspace.svelte";
    import type { Context } from "quill/modules/keyboard";
    

    let titleText: string = $state("");
    // setContext(
    //     'title',
    //     {
    //         updateTitleText,
    //         returnTitleText
    //     }
    // );
    export function updateTitleText(Text: string) {
        titleText = Text;
    }
    export function returnTitleText(): string {
        return titleText;
    }

    export function handleTitleChange(event: Event): void {
        const target = event.target as HTMLTextAreaElement;
        titleText = target.value;
    }
</script>

<main class="flex justify-center">
    <div class="flex w-[60%] h-[10%] mx-auto">
        <textarea
            class="w-full h-full resize-none border-none bg-base rounded-lg text-text focus:outline-none focus:ring-0"
            placeholder="Enter Title here..."
            value={titleText}
            oninput={handleTitleChange}
        ></textarea>
    </div>
</main>
