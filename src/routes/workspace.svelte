<script lang="ts" module>
    import Tabsbar from "../lib/components/tabsbar.svelte";
    import TitleBox from "../lib/components/titlebox.svelte";
    import Editor from "../lib/components/editor.svelte"
    import { setContext, getContext } from 'svelte';
    import { autoSave, loadRecentDocuments, handleKeydown } from '../lib/functions/functions.svelte';  
    
    interface Document {
        id: string;
        title: string;
        content: string;
    }

    let currentId: string = $state("");
    let isCommandPalettevisible: boolean = $state(false);
    
    
    // setContext(
    //     'workspace',
    //     {
    //         updateCurrentID,
    //         getCurrentID,
    //         toggleCommandPalette
    //     }
    // );

    // const io: any = getContext('io');

    export function updateCurrentID(id: string) {
        currentId = id;
    }

    export function getCurrentID(): string {
        return currentId;
    }

    export function toggleCommandPalette(): void {
        isCommandPalettevisible = !isCommandPalettevisible;
    }
</script>
<svelte:window on:keydown={handleKeydown} />

<main>
    <Tabsbar/>
    <div class="flex flex-col justify-center mt-[6%] w-full">
        <TitleBox/>
        <Editor/>
    </div>
</main>