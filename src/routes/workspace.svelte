<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import CommandPalette from "../lib/components/commandpalette.svelte";
    import { getContext, onMount, setContext } from 'svelte';
    import { X, Minus, Square } from 'lucide-react';
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
    import Blockquote from '@tiptap/extension-blockquote';
    import Typography from '@tiptap/extension-typography';
    import Document from '@tiptap/extension-document'
    import Paragraph from '@tiptap/extension-paragraph';
    import Text from '@tiptap/extension-text';
    import Heading from '@tiptap/extension-heading';
    import Code from '@tiptap/extension-code';
    import CodeBlock from '@tiptap/extension-code-block';
    import Italic from '@tiptap/extension-italic';
    import HorizontalRule from '@tiptap/extension-horizontal-rule';
    // import OrderedList from '@tiptap/extension-ordered-list';
    // import BulletList from '@tiptap/extension-bullet-list';
    import Close from '$lib/static/close.svg'
    import Minimise from '$lib/static/minimise.svg'
    import Maximise from '$lib/static/maximise.svg'
    

    interface DocumentData {
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
    let recentDocuments:  DocumentData[] = [];
    let isCommandPalettevisible: boolean = $state(false);

    setContext(
        'editor', 
        {   
            addnewtab, 
            newDocument,
            switchTab, 
            gotoLastTab, 
            gotoTab1, 
            cycleTabs, 
            deleteDocument,
            return_isCommandPalettevisible: () => isCommandPalettevisible, 
            toggleCommandPalette 
    });

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

    const appWindow = getCurrentWindow();
    
    onMount(() => {
        editor = new Editor({
        element,
        extensions: [
            StarterKit,
            Heading.configure({
                HTMLAttributes: {
                    class: 'text-text'
                }
            }),
            TextStyle,
            Blockquote.configure({
                HTMLAttributes: {
                    class: 'border-l-2 border-current pl-4 my-4 text-text text-sm bg-transparent font-normal leading-none before:content-none after:content-none'
                }
            }),
            Document,
            Bold.configure({
                HTMLAttributes: {
                    class: 'text-text font-bold'
                }
            }),
            Color,
            FontSizeTextStyle,
            FontFamily,
            Highlight,
            Text,
            HorizontalRule,
            Paragraph,
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
            YouTube,
            Typography,
            Code.configure({
            HTMLAttributes: {
                    class: 'bg-mantle text-text rounded px-1'  // Added bg-surface0
                }
            }),
            CodeBlock.configure({
                HTMLAttributes: {
                    class: 'bg-mantle text-text rounded-lg p-4'  // Added bg-surface0
                }
            }),
            Italic
        ],
        content: ``,
        editorProps: {
            attributes: {
            class: 'format  text-text text-sm focus:outline-none format-blue max-w-none leading-none'
            }
        },
        onUpdate: ({ editor }) => {
        updatecharwordcounts();
        }
        });

        loadRecentDocuments().then(async () => {
            currentTabs = await getTabs();
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

    function updatecharwordcounts() {
        charCount = editor.storage.characterCount.characters();
        wordCount = editor.storage.characterCount.words();
    }

    export function toggleCommandPalette(): void {
        isCommandPalettevisible = !isCommandPalettevisible;
    }

    async function getTabs(): Promise<Tab[]> {
        return await invoke("get_tabs"); 
    }

    async function addnewtab(): Promise<void> {
        const newTab: Tab = await invoke("new_tab");
        currentId = newTab.id;
        titleText = newTab.title;
        editor.commands.setContent('');
        updatecharwordcounts();
        currentTabs = await getTabs();
        await invoke("send_current_open_tab", { id: newTab.id });
    }

    async function switchTab(tabId: string): Promise<void> {
        try {
            const docResult:  DocumentData | null = await invoke(
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
            updatecharwordcounts();
        } catch (error) {
            console.error("Failed to switch tab:", error);
        }
    }

    async function cycleTabs(): Promise<void> {
        try {
            const nextTabId: string = await invoke("cycle_tabs");
            const docResult:  DocumentData | null = await invoke("get_document_content", { id: nextTabId });
            
            if (docResult) {
                currentId = nextTabId;
                titleText = docResult.title;
                editor.commands.setContent(docResult.content);
                updatecharwordcounts();
            }
        } catch (error) {
            console.error("Failed to cycle tabs:", error);
        }
    }

    async function gotoTab1(): Promise<void> {
        const tabs = await getTabs();
        if (tabs.length > 0) {
            await switchTab(tabs[0].id);
        }
    }

    async function gotoLastTab(): Promise<void> {
        const tabs = await getTabs();
        if (tabs.length > 0) {
            await switchTab(tabs[tabs.length - 1].id);
        }
    }

    async function autoSave(): Promise<void> {
        if (!titleText && !editor!.getText()) return;

        try {
            await invoke("update_tab_title", {
                id: currentId,
                title: titleText,
            });
            currentTabs = await getTabs();
            await invoke("save_document", {
                id: currentId,
                title: titleText,
                content: editor.getHTML(),
            });
        } catch (error) {
            console.error("Auto-save failed:", error);
        }
    }

    async function loadRecentDocuments(): Promise<void> {
        try {
            const docs:  DocumentData[] = await invoke("load_recent_files");
            recentDocuments = docs;

            // Update the tabs in UI
            currentTabs = await getTabs();

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

    async function deleteDocument(): Promise<void> {
        try {
            // The Rust function returns the next document's content after deletion
            const nextDoc:  DocumentData | null = await invoke("delete_document", { id: currentId });
            currentTabs = await getTabs();
            
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

    async function newDocument(): Promise<void> {
        try {
            const newTab: Tab = await invoke("new_tab");
            currentId = newTab.id;
            titleText = newTab.title;
            editor!.commands.setContent("");
            updatecharwordcounts();
            currentTabs = await getTabs();
        } catch (error) {
            console.error("Failed to create new document:", error);
        }
    }

    function handleKeydown(event: KeyboardEvent): void {
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

    function handleTitleChange(event: Event): void {
        const target = event.target as HTMLTextAreaElement;
        titleText = target.value;
    }
</script>
<svelte:window on:keydown={handleKeydown} />

<main>
    <div data-tauri-drag-region class="fixed flex bg-base top-[0px] w-full h-[40px] select-none justify-between px-1" role="tablist" aria-label="Document tabs" >
        <div data-tauri-drag-region class="flex flex-row items-center h-full px-4 flex-grow">
            {#each currentTabs as tab}
                <button
                    type="button"
                    class={`flex justify-left items-center px-4 text-nowrap h-[30px] min-w-[120px] rounded-[18px] flex-shrink text-text m-[0.6%] hover:bg-surface1 ${currentId === tab.id ? 'bg-surface0' : ''}`}
                    role="tab"
                    aria-controls="editor"
                    onclick={() => switchTab(tab.id)}
                >
                    {tab.title.length > 20 ? tab.title.slice(0, 20) + '...' : tab.title || 'Untitled'}
                </button>
            {/each}
        </div>
        <div class="flex flex-row items-center h-full">
            <button class="titlebar-button h-full px-3 cursor-pointer hover:bg-surface2" id="titlebar-minimize" onclick={() => appWindow.minimize()} aria-label="Minimize">
                <img
                    src={ Minimise }
                    alt="minimize"
                />
            </button>
            <button class="titlebar-button h-full px-3 cursor-pointer hover:bg-surface2" id="titlebar-maximize" onclick={() => appWindow.toggleMaximize()} aria-label="Maximise">
                <img
                    src={ Maximise }
                    alt="maximize"
                />
            </button>
            <button class="titlebar-button h-full px-3 cursor-pointer hover:bg-red-700" id="titlebar-close" onclick={() => appWindow.close()} aria-label="Close">
                <img
                    src={ Close }
                    alt="close"
                />
            </button>
        </div>
    </div>
    <div class="flex flex-col justify-start mt-[60px] h-[calc(100vh-60px)] overflow-hidden">
    <main class="flex h-[80px] mb-5">
        <div class="flex w-[50%] h-full mx-auto">
            <textarea
                class="w-full h-full resize-none border-none bg-base rounded-lg py-7 text-text text-[2rem] focus:outline-none focus:ring-0 shadow-lg"
                placeholder="Enter Title here..."
                value={titleText}
                oninput={handleTitleChange}
            ></textarea>
        </div>
    </main>
    <main class="flex-1 min-h-0 overflow-hidden mb-20 p-2">
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="h-full w-[70%] rounded-lg m-[0.5%] mx-auto cursor-text">
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <div 
                class="h-full w-full overflow-auto custom-scrollbar p-2"
                bind:this={element}
                onclick={() => editor.commands.focus()}
            ></div>
        </div>
    </main>
</div>
    <div class="fixed flex flex-row gap-[20px] self-end bottom-[10px] right-[10px] bg-base px-[10px] py-[5px] rounded-[18px] z-10 text-text text-[0.85em] select-none">
        <div>{wordCount} Words</div>
        <div>{charCount} Characters</div>
    </div>
    <CommandPalette/>
</main>

<style>
    .custom-scrollbar {
      height: 100%;
      padding-bottom: 8px;
    }
</style>