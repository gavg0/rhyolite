<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { getContext } from 'svelte';
    import Workspace from "../../routes/workspace.svelte";
    import type { Context } from "quill/modules/keyboard";
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
