<script lang="ts">
    import {Input, TabItem,} from "flowbite-svelte";
    import type {Tab} from "../types/tab";
    import ContentEditor from "./content-editor.svelte";
    import {Editor,} from 'svelte-tiptap';
    import {onMount} from "svelte";
    import TabService from "../services/tab.service";
    import DocumentService from "../services/document.service";

interface DocumentTabItemProps {
    open?: boolean;
    tab: Tab;
    onclick: (event: Event) => void;
}

let { open, tab, onclick }: DocumentTabItemProps = $props();

let documentTitle: string = $state("");
let documentContent: any = $state();
onMount(async () => {
    documentTitle = tab.title;
    // content = tab.content;
    console.log('load...');
    const doc = await DocumentService.loadDocument(tab.id);

    if(!doc) return;
    documentContent = doc.content;
    documentTitle = doc.title;
    await TabService.updateTabTitleById(tab.id, documentTitle);
});

const handleTitleChange = (event: Event) => {
    const target = event.target as HTMLTextAreaElement;
    documentTitle = target.value;
    TabService.updateTabTitleById(tab.id, documentTitle);

    saveDocument();
}

let saveTimeout: number | undefined;
const delaySave = 500;
const handleContentChange = (editor: Editor) => {
    documentContent = editor.getJSON();

    saveDocument();
}

const saveDocument = async () => {
    // Clear the previous timeout
    if(saveTimeout) clearTimeout(saveTimeout);
    // Set a new timeout to trigger `saveAction` after 0.5 seconds
    saveTimeout = setTimeout(() => {
        console.log('saving...');
        DocumentService.saveDocument({
            documentId: tab.id,
            documentTitle,
            documentContent,
        });
    }, delaySave ?? 500);
}
</script>

<TabItem open={open} onclick={onclick}>
    <span slot="title">{ tab.title }</span>
    <div class="mb-6">
        <Input id="large-input" size="lg" placeholder="Enter title here..." value={documentTitle} oninput={handleTitleChange} />
    </div>
    <div class="mb-6">
        <ContentEditor content={documentContent} onchange={handleContentChange}  />
    </div>
</TabItem>