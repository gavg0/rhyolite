<script lang="ts">
    import Tabsbar from "../lib/components/tabsbar.svelte";
    import TitleBox from "../lib/components/titlebox.svelte";
    import Editor from "../lib/components/editor.svelte"
    import { setContext, getContext } from 'svelte';
    
    interface Document {
        id: string;
        title: string;
        content: string;
    }

    let currentId: string = $state("");
    let isCommandPalettevisible: boolean = $state(false);
    
    
    setContext(
        'workspace',
        {
            updateCurrentID,
            getCurrentID,
            toggleCommandPalette
        }
    );

    const io: any = getContext('io');

    function updateCurrentID(id: string) {
        currentId = id;
    }

    function getCurrentID(): string {
        return currentId;
    }

    function toggleCommandPalette(): void {
        isCommandPalettevisible = !isCommandPalettevisible;
    }
</script>
<svelte:window on:keydown={io.handleKeydown} />

<main>
    <Tabsbar/>
    <div class="flex flex-col justify-center mt-[6%] w-full">
        <TitleBox/>
        <Editor/>
    </div>
</main>