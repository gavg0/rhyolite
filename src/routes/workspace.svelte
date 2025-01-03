<script lang="ts">
    import Tabsbar from "../lib/components/tabsbar.svelte";
    import TitleBox from "../lib/components/titlebox.svelte";
    import Editor from "../lib/components/editor.svelte"
    import { setContext } from 'svelte';
    
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
            getCurrentID
        }
    );

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

<main>
    <Tabsbar/>
    <div class="flex flex-col mt-[6%] w-full">
        
        <Editor/>
    </div>
</main>