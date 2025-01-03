<script lang="ts">
    import Tabsbar from "../lib/components/tabsbar.svelte";
    import TitleBox from "../lib/components/titlebox.svelte";
    import Editor from "../lib/components/editor.svelte"
    import { setContext } from 'svelte';
    

    let currentId: string = $state("");
    setContext(
        'workspace',
        {
            updateCurrentID
        }
    );

    function updateCurrentID(id: string) {
        currentId = id;
    }
</script>

<main>
    <Tabsbar/>
    <div class="flex flex-col mt-[6%] w-full">
        
        <Editor/>
    </div>
</main>