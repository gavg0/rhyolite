<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    // import CommandPalette from "../lib/components/commandpalette.svelte";
    import { getContext, onMount, setContext } from 'svelte';
    import { Editor } from '@tiptap/core';
    import StarterKit from '@tiptap/starter-kit';
    import Highlight from '@tiptap/extension-highlight';
    import Underline from '@tiptap/extension-underline';
    import Link from '@tiptap/extension-link';
    import TextAlign from '@tiptap/extension-text-align';
    import Image from '@tiptap/extension-image';
    import YouTube from '@tiptap/extension-youtube';
    import TextStyle from '@tiptap/extension-text-style';
    import FontFamily from '@tiptap/extension-font-family';
    import CharacterCount from '@tiptap/extension-character-count'
    import { Color } from '@tiptap/extension-color';
    import Bold from '@tiptap/extension-bold';
    
    interface Document {
        id: string;
        title: string;
        content: string;
    }

    interface Tab {
        id: string;
        title: string;
    }

    let editor: Editor;
    let element: Element;
    let titleText: string = $state("");
    let currentTabs: Tab[] = $state([]);
    let currentTabID: string = $state("");
    let wordCount: number = $state(0);
    let charCount: number = $state(0);
    let currentId: string = $state("");
    let recentDocuments: Document[] = [];
    let isCommandPalettevisible: boolean = $state(false);

    const FontSizeTextStyle = TextStyle.extend({
        addAttributes() {
        return {
            fontSize: {
            default: null,
            parseHTML: element => element.style.fontSize,
            renderHTML: attributes => !attributes.fontSize ? {} : { style: `font-size: ${attributes.fontSize}` }
            }
        };
        }
    });
    
    const CustomBold = Bold.extend({
        renderHTML({ mark, HTMLAttributes }) {
        const { style, ...rest } = HTMLAttributes;
        const newStyle = `font-weight: bold;${style ? ' ' + style : ''}`;
        return ['span', { ...rest, style: newStyle.trim() }, 0];
        },
        addOptions() {
        return {
            ...this.parent?.(),
            HTMLAttributes: {}
        };
        }
    });
    
    onMount(() => {
        editor = new Editor({
        element,
        extensions: [
            StarterKit.configure({
            bold: false,
            }),
            CustomBold,
            TextStyle,
            Color,
            FontSizeTextStyle,
            FontFamily,
            Highlight,
            Underline,
            Link.configure({
            openOnClick: false,
            autolink: true,
            defaultProtocol: 'https'
            }),
            TextAlign.configure({
            types: ['heading', 'paragraph']
            }),
            CharacterCount.configure({
            textCounter: (text) => [...new Intl.Segmenter().segment(text)].length,
            wordCounter: (text) => text.split(/\s+/).filter((word) => word !== '').length
            }),
            Image,
            YouTube
        ],
        content: ``,
        editorProps: {
            attributes: {
            class: 'format lg:format-lg text-text text-sm focus:outline-none format-blue max-w-none leading-none'
            }
        },
        onUpdate: ({ editor }) => {
        updatecharwordcounts();
        }
        });

        loadRecentDocuments().then(async () => {
                await updateTabs();
                let currentTabs = returnTabsArray();
                if (currentTabs.length === 0) {
                    await addnewtab();
                    updatecharwordcounts();
                }
            });
        
        // Set up auto-save
        const autoSaveInterval = setInterval(autoSave, 500);
    
        return () => {
        editor.destroy();
        clearInterval(autoSaveInterval);
        };
    });
    
    function toggleBold() { editor.chain().focus().toggleBold().run(); }
    function toggleItalic() { editor.chain().focus().toggleItalic().run(); }
    function toggleHighlight() {
        const isHighlighted = editor.isActive('highlight');
        editor.chain().focus().toggleHighlight({
        color: isHighlighted ? 'Transparent' : '#ffc078'
        }).run();
    }

    function setEditorContent(Content: any) {
        editor.commands.setContent(Content);
    }

    function getEditorContent(): any {
    return editor.getHTML();
    }

    function getEditorContentasText(): string {
        return editor.getText();
    }

    function updatecharwordcounts() {
        charCount = editor.storage.characterCount.characters();
        wordCount = editor.storage.characterCount.words();
    }

    export function toggleCommandPalette(): void {
        isCommandPalettevisible = !isCommandPalettevisible;
    }
    
    export function return_isCommandPalettevisible(): boolean {
        return isCommandPalettevisible;
    }

    export function returnTabsArray(): Tab[] {
        return currentTabs;
    }

    export async function getTabs(): Promise<Tab[]> {
        return await invoke("get_tabs"); // New Rust function needed
    }

    export async function updateTabs(): Promise<void> {
        currentTabs = await getTabs();
    }

    export async function addnewtab(): Promise<void> {
        const newTab: Tab = await invoke("new_tab");
        currentId = newTab.id;
        titleText = newTab.title;
        editor.commands.setContent('');
        await updateTabs();
        await invoke("send_current_open_tab", { id: newTab.id });
    }

    export async function switchTab(tabId: string): Promise<void> {
        try {
            const docResult: Document | null = await invoke(
                "get_document_content",
                { id: tabId }
            );

            if (docResult) {
                currentId = tabId;
                currentTabID = tabId;
                titleText = docResult.title;
                editor.commands.setContent(docResult.content);
            } else {
                currentId = tabId;
                currentTabID = tabId;
                titleText = "";
                editor.commands.setContent("");
            }
            
            // Update the current open tab in the backend
            await invoke("send_current_open_tab", { id: tabId });
        } catch (error) {
            console.error("Failed to switch tab:", error);
        }
    }

    export async function cycleTabs(): Promise<void> {
        try {
            const nextTabId: string = await invoke("cycle_tabs");
            const docResult: Document | null = await invoke("get_document_content", { id: nextTabId });
            
            if (docResult) {
                currentId = nextTabId;
                titleText = docResult.title;
                editor.commands.setContent(docResult.content);
            }
        } catch (error) {
            console.error("Failed to cycle tabs:", error);
        }
    }

    export async function gotoTab1(): Promise<void> {
        const tabs = await getTabs();
        if (tabs.length > 0) {
            await switchTab(tabs[0].id);
        }
    }

    export async function gotoLastTab(): Promise<void> {
        const tabs = await getTabs();
        if (tabs.length > 0) {
            await switchTab(tabs[tabs.length - 1].id);
        }
    }

    export async function autoSave(): Promise<void> {
        if (!titleText && !editor!.getText()) return;

        try {
            await invoke("update_tab_title", {
                id: currentId,
                title: titleText,
            });
            await updateTabs();
            await invoke("save_document", {
                id: currentId,
                title: titleText,
                content: editor.getHTML(),
            });
        } catch (error) {
            console.error("Auto-save failed:", error);
        }
    }

    export async function loadRecentDocuments(): Promise<void> {
        try {
            const docs: Document[] = await invoke("load_recent_files");
            recentDocuments = docs;

            // Update the tabs in UI
            await updateTabs();

            if (recentDocuments.length > 0) {
                // Load the last open tab from the backend
                const openTabId: string = await invoke("get_current_open_tab");
                await switchTab(openTabId);
            } else {
                // If no documents exist, create a new tab
                await addnewtab();
            }
        } catch (error) {
            console.error("Failed to load documents:", error);
        }
    }

    export async function deleteDocument(): Promise<void> {
        try {
            // The Rust function returns the next document's content after deletion
            const nextDoc: Document | null = await invoke("delete_document", { id: currentId });
            await updateTabs();
            
            if (nextDoc) {
                // If we have a next document, switch to it
                currentId = nextDoc.id;
                titleText = nextDoc.title;
                editor!.commands.setContent(nextDoc.content);
            } else {
                // If no documents left, create a new one
                await addnewtab();
            }
            await invoke("send_current_open_tab", { id: currentId });
        } catch (error) {
            console.error("Failed to delete document:", error);
        }
    }

    export async function newDocument(): Promise<void> {
        try {
            const newTab: Tab = await invoke("new_tab");
            currentId = newTab.id;
            titleText = newTab.title;
            editor!.commands.setContent("");
            await updateTabs();
        } catch (error) {
            console.error("Failed to create new document:", error);
        }
    }

    export function handleKeydown(event: KeyboardEvent): void {
        if (event.ctrlKey && event.key === "d") {
            event.preventDefault();
            deleteDocument();
        }
        if (event.ctrlKey && event.key === "n") {
            event.preventDefault();
            newDocument();
        }
        // if (event.ctrlKey && event.key === "t") {
        //     event.preventDefault();
        //     toggleToolbar();
        // }
        if (
            (event.ctrlKey && event.key === "Tab") ||
            (event.ctrlKey && event.key === "PageDown")
        ) {
            event.preventDefault();
            cycleTabs();
        }
        if (event.ctrlKey && event.key === "1") {
            event.preventDefault();
            gotoTab1();
        }
        if (event.ctrlKey && event.key === "9") {
            event.preventDefault();
            gotoLastTab();
        }
        if (event.ctrlKey && event.key === "p") {
            event.preventDefault();
            toggleCommandPalette();
        }
    }

    export function handleTitleChange(event: Event): void {
        const target = event.target as HTMLTextAreaElement;
        titleText = target.value;
    }
</script>
<svelte:window on:keydown={handleKeydown} />

<main>
    <div class="fixed bg-base top-[0px] w-full h-[45px]" role="tablist" aria-label="Document tabs">
        <div class="flex flex-row ml-1">
            {#each currentTabs as tab}
                <button
                    type="button"
                    class="flex justify-left items-center p-[1%] h-[30px] w-auto rounded-[18px] flex-shrink active:bg-crust  text-text m-[0.6%]"
                    class:active={currentTabID === tab.id}
                    role="tab"
                    aria-controls="editor"
                    onclick={() => switchTab(tab.id)}
                >
                    {tab.title.length > 10 ? tab.title.slice(0, 20) + '...' : tab.title || 'Untitled'}
                </button>
            {/each}
        </div>
    </div>
    <div class="flex flex-col justify-start mt-[60px] h-[100vh] flex-grow">
        <main class="flex h-[80px] mb-5">
            <div class="flex w-[50%] h-full mx-auto">
                <textarea
                    class="w-full h-full resize-none border-none bg-base rounded-lg py-7 text-text text-[2.5rem] focus:outline-none focus:ring-0"
                    placeholder="Enter Title here..."
                    value={titleText}
                    oninput={handleTitleChange}
                ></textarea>
            </div>
        </main>
        <main class="h-full overflow-hidden">
            <div class="flex rounded-lg  m-[0.5%] w-[70%] h-full flex-grow cursor-text mx-auto">
              <div class="w-full h-full" bind:this={element}></div>
            </div>
        </main>
    </div>
    <div class="fixed flex flex-row gap-[20px] self-end bottom-[10px] right-[10px] bg-base px-[10px] py-[5px] rounded-[18px] z-10 text-text text-[0.85em]">
        <div>{wordCount} Words</div>
        <div>{charCount} Characters</div>
    </div>
    
</main>