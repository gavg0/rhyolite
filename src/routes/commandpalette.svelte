<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    let isvisible: boolean = $state(false);
    let selectedindex: number = $state(0);

    interface Command {
        name: String;
        shortcut: String;
        action: () => void;
    }

    // let commands: Command[] = [
    //   {
    //     name: 'Delete Tab',
    //     shortcut: 'Ctrl + D',
    //     action: () => deleteDocument
    //   }
    // ];
</script>
