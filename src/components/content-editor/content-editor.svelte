<script lang="ts">
    import {onDestroy, onMount} from 'svelte';
    import type { Readable } from 'svelte/store';
    import { createEditor, Editor, EditorContent } from 'svelte-tiptap';
    import { Dropdown, DropdownItem, Tooltip } from 'flowbite-svelte';
    import StarterKit from '@tiptap/starter-kit';
    import Highlight from '@tiptap/extension-highlight';
    import Underline from '@tiptap/extension-underline';
    import Link from '@tiptap/extension-link';
    import TextAlign from '@tiptap/extension-text-align';
    import Image from '@tiptap/extension-image';
    import YouTube from '@tiptap/extension-youtube';
    import TextStyle from '@tiptap/extension-text-style';
    import FontFamily from '@tiptap/extension-font-family';
    import { Color } from '@tiptap/extension-color';
    import Bold from '@tiptap/extension-bold';
    import type { ChainedCommands } from "@tiptap/core";
    import ContentEditorStore from "../../stores/content-editor.store";
    import { ChevronUpOutline, ChevronDownOutline } from 'flowbite-svelte-icons';
    import ToolbarButton from "./components/toolbar-button.svelte";

    let editor = $state() as Readable<Editor>;

    interface ContentEditorProps {
        content: any;
        onchange: (content: Editor) => void;
    }
    const { content, onchange }: ContentEditorProps = $props();

    const setupEditor = () => {
        const FontSizeTextStyle = TextStyle.extend({
            addAttributes() {
                return {
                    fontSize: {
                        default: null,
                        renderHTML: attributes => {
                            if (!attributes.fontSize) {
                                return {};
                            }
                            return {style: `font-size: ${attributes.fontSize}`};
                        },
                    },
                };
            },
        });

        const CustomBold = Bold.extend({
            renderHTML({ HTMLAttributes }) {
                const {style, ...rest} = HTMLAttributes;
                const newStyle = `font-weight: bold; ${style || ''}`.trim();
                return ['span', {...rest, style: newStyle}, 0];
            },
            addOptions() {
                return {
                    ...this.parent?.(),
                    HTMLAttributes: {},
                };
            },
        });

        editor = createEditor({
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
                    defaultProtocol: 'https',
                }),
                TextAlign.configure({
                    types: [
                        'heading',
                        'paragraph',
                    ]
                }),
                Image,
                YouTube,
            ],
            content,
            editorProps: {
                attributes: {
                    class: 'format lg:format-lg dark:format-invert focus:outline-none format-blue max-w-none',
                },
            },
            // onTransaction: ({ editor }) => {
                // // force re-render so `editor.isActive` works as expected
                // editor = editor;

            // },
            onUpdate: ({ editor }) => {
                onchange(editor as Editor);
            }
        });
    };

    onMount(() => {
        setupEditor();
    });

    const toggleMark = (mark: string) => {
        const focused: ChainedCommands = $editor.chain().focus();
        if (typeof (focused as any)[mark] === 'function') {
            (focused as any)[mark]().run();
        }
    };

    const toggleLink = () => {
        const url: string | null = window.prompt('Enter image URL:', 'https://');
        if(url)
            $editor.chain().focus().toggleLink({ href: url }).run();
    };

    const addImage = () => {
        const url = window.prompt('Enter image URL:', 'https://');
        if (url) {
            $editor.chain().focus().setImage({ src: url }).run();
        }
    };

    const addYoutubeVideo = () => {
        const url = window.prompt('Enter YouTube URL:', 'https://www.youtube.com/watch?v=');
        if (url) {
            $editor.commands.setYoutubeVideo({
                src: url,
                width: 640,
                height: 480,
            })
        }
    };

    let textSizeDropdownOpen = $state(false);
    const setTextSize = ( event: Event ) => {
        const fontSize: string | null = (event.target as HTMLInputElement).getAttribute('data-text-size');
        // Apply the selected font size via pixels using the TipTap editor chain
        if(fontSize) $editor.chain().focus().setMark('textStyle', { fontSize }).run();
        //close dropdown after selection font size
        textSizeDropdownOpen = false;
    };

    let colorDropdownOpen = $state(false);
    const onInputColor = (event: Event) => {
        const selectedColor = (event.target as HTMLInputElement).value;
        // Apply the selected color to the selected text
        $editor.chain().focus().setColor(selectedColor).run();
        colorDropdownOpen = false;
    };
    const onClickColorHex = (event: Event) => {
        const selectedColor: string | null = (event.target as HTMLInputElement).getAttribute('data-hex-color');
        // Apply the selected color to the selected text
        if (selectedColor) $editor.chain().focus().setColor(selectedColor).run();
        colorDropdownOpen = false;
    }

    const resetColor = () => {
        $editor.commands.unsetColor();
    }

    let fontFamilyDropdownOpen = $state(false);
    const onSelectFontFamily = (event: Event) => {
        const fontFamily: string | null = (event.target as HTMLInputElement).getAttribute('data-font-family');

        // Apply the selected font size via pixels using the TipTap editor chain
        if(fontFamily) $editor.chain().focus().setFontFamily(fontFamily).run();

        // Hide the dropdown after selection
        fontFamilyDropdownOpen = false;
    }

    const setTextAlign = ( align: 'left' | 'right' | 'center') => {
        $editor.chain().focus().setTextAlign(align).run();
    }

    let typographyDropdownOpen = $state(false);
    const onSelectHeading = (event: Event) => {
        console.log(event);
        const headingLevel: string | null = (event.target as HTMLInputElement).getAttribute('data-heading-level');
        console.log(headingLevel);
        if(headingLevel) {
            const level = parseInt(headingLevel) as 1 | 2 | 3 | 4 | 5 | 6;
            $editor.chain().focus().toggleHeading({ level }).run();
        }
        typographyDropdownOpen = false;
    }
    const onSelectParagraph = () => {
        $editor.chain().focus().setParagraph().run();
        typographyDropdownOpen = false;
    }

    let flagToolbarVisibility = $state(false);
    const unsubscribeStates = ContentEditorStore.states.subscribe(value => {
        flagToolbarVisibility = value.flagToolbarVisibility;
    });
    onDestroy(unsubscribeStates); // Clean up

</script>

<div class="w-full border border-gray-200 rounded-lg bg-gray-50 dark:bg-gray-700 dark:border-gray-600">
    <div class="flex justify-center { flagToolbarVisibility ? 'border-b dark:border-gray-600' : '' }">
        <button class="w-full flex justify-center" onclick={ContentEditorStore.toggleToolbarVisibility}>
            {#if flagToolbarVisibility}
                <ChevronUpOutline />
            {:else}
                <ChevronDownOutline />
            {/if}
        </button>
        <Tooltip>
            {#if flagToolbarVisibility}
                Hide toolbar
            {:else}
                Show toolbar
            {/if}
        </Tooltip>
    </div>
    <div class="border-b dark:border-gray-600 px-3">
        <div class:hidden={!flagToolbarVisibility}>
            <div class="flex flex-wrap items-center">
                <div class="flex items-center space-x-1 rtl:space-x-reverse flex-wrap">
                    <ToolbarButton onclick={() => toggleMark('toggleBold')} tooltipText="Toggle bold">
                        <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 5h4.5a3.5 3.5 0 1 1 0 7H8m0-7v7m0-7H6m2 7h6.5a3.5 3.5 0 1 1 0 7H8m0-7v7m0 0H6"/>
                        </svg>
                    </ToolbarButton>
                    <ToolbarButton onclick={() => toggleMark('toggleItalic')} tooltipText="Toggle italic">
                        <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m8.874 19 6.143-14M6 19h6.33m-.66-14H18"/>
                        </svg>
                    </ToolbarButton>
                    <ToolbarButton onclick={() => toggleMark('toggleUnderline')} tooltipText="Toggle underline">
                        <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                            <path stroke="currentColor" stroke-linecap="round" stroke-width="2" d="M6 19h12M8 5v9a4 4 0 0 0 8 0V5M6 5h4m4 0h4"/>
                        </svg>
                    </ToolbarButton>
                    <ToolbarButton onclick={() => toggleMark('toggleStrike')} tooltipText="Toggle strike">
                        <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 6.2V5h12v1.2M7 19h6m.2-14-1.677 6.523M9.6 19l1.029-4M5 5l6.523 6.523M19 19l-7.477-7.477"/>
                        </svg>
                    </ToolbarButton>
                    <ToolbarButton onclick={() => toggleMark('toggleHighlight')} tooltipText="Toggle highlight">
                        <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                            <path stroke="currentColor" stroke-linecap="round" stroke-width="2" d="M9 19.2H5.5c-.3 0-.5-.2-.5-.5V16c0-.2.2-.4.5-.4h13c.3 0 .5.2.5.4v2.7c0 .3-.2.5-.5.5H18m-6-1 1.4 1.8h.2l1.4-1.7m-7-5.4L12 4c0-.1 0-.1 0 0l4 8.8m-6-2.7h4m-7 2.7h2.5m5 0H17"/>
                        </svg>
                    </ToolbarButton>
                    <ToolbarButton onclick={() => toggleMark('toggleCode')} tooltipText="Format code">
                        <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m8 8-4 4 4 4m8 0 4-4-4-4m-2-3-4 14"/>
                        </svg>
                    </ToolbarButton>
                    <ToolbarButton onclick={toggleLink} tooltipText="Add link">
                        <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.213 9.787a3.391 3.391 0 0 0-4.795 0l-3.425 3.426a3.39 3.39 0 0 0 4.795 4.794l.321-.304m-.321-4.49a3.39 3.39 0 0 0 4.795 0l3.424-3.426a3.39 3.39 0 0 0-4.794-4.795l-1.028.961"/>
                        </svg>
                    </ToolbarButton>
                    <ToolbarButton onclick={() => toggleMark('unsetLink')} tooltipText="Remove link">
                        <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                            <path stroke="currentColor" stroke-linecap="round" stroke-width="2" d="M13.2 9.8a3.4 3.4 0 0 0-4.8 0L5 13.2A3.4 3.4 0 0 0 9.8 18l.3-.3m-.3-4.5a3.4 3.4 0 0 0 4.8 0L18 9.8A3.4 3.4 0 0 0 13.2 5l-1 1m7.4 14-1.8-1.8m0 0L16 16.4m1.8 1.8 1.8-1.8m-1.8 1.8L16 20"/>
                        </svg>
                    </ToolbarButton>
                    <button
                            id="toggleTextSizeButton"
                            type="button"
                            class="custom-btn-toolbar"
                    >
                        <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 6.2V5h11v1.2M8 5v14m-3 0h6m2-6.8V11h8v1.2M17 11v8m-1.5 0h3"/>
                        </svg>
                        <span class="sr-only">Text size</span>
                    </button>
                    <Dropdown  bind:open={ textSizeDropdownOpen } class="w-72 rounded bg-white p-2 shadow dark:bg-gray-700">
                        {#each [
                            { size: "16px", label: "16px (Default)", className: "text-base" },
                            { size: "12px", label: "12px (Tiny)", className: "text-xs" },
                            { size: "14px", label: "14px (Small)", className: "text-sm" },
                            { size: "18px", label: "18px (Lead)", className: "text-lg" },
                            { size: "24px", label: "24px (Large)", className: "text-2xl" },
                            { size: "36px", label: "36px (Huge)", className: "text-4xl" }
                        ] as { size, label, className }}
                            <DropdownItem class={className}>
                                <button
                                        data-text-size={size}
                                        type="button"
                                        onclick={setTextSize}
                                        class="flex justify-between items-center w-full rounded px-3 py-2"
                                >
                                    {label}
                                </button>
                            </DropdownItem>
                        {/each}
                    </Dropdown>
                    <Tooltip triggeredBy="#toggleTextSizeButton">
                        Text size
                    </Tooltip>
                    <button id="toggleTextColorButton" type="button" class="custom-btn-toolbar">
                        <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="25" height="24" fill="none" viewBox="0 0 25 24">
                            <path stroke="currentColor" stroke-linecap="round" stroke-width="2" d="m6.532 15.982 1.573-4m-1.573 4h-1.1m1.1 0h1.65m-.077-4 2.725-6.93a.11.11 0 0 1 .204 0l2.725 6.93m-5.654 0H8.1m.006 0h5.654m0 0 .617 1.569m5.11 4.453c0 1.102-.854 1.996-1.908 1.996-1.053 0-1.907-.894-1.907-1.996 0-1.103 1.907-4.128 1.907-4.128s1.909 3.025 1.909 4.128Z"/>
                        </svg>
                        <span class="sr-only">Text color</span>
                    </button>
                    <Dropdown bind:open={ colorDropdownOpen } class="w-48 rounded bg-white p-2 shadow dark:bg-gray-700">
                        <div class="grid grid-cols-6 gap-2 group mb-3 items-center p-1.5 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-600">
                            <input
                                    type="color"
                                    id="color"
                                    value="#e66465"
                                    oninput={ onInputColor }
                                    class="border-gray-200 border bg-gray-50 dark:bg-gray-700 dark:border-gray-600 rounded-md p-px px-1 hover:bg-gray-50 group-hover:bg-gray-50 dark:group-hover:bg-gray-700 w-full h-8 col-span-3"
                            />
                            <label for="color" class="text-gray-500 dark:text-gray-400 text-sm font-medium col-span-3 group-hover:text-gray-900 dark:group-hover:text-white">Pick a color</label>
                        </div>
                        <div class="grid grid-cols-6 gap-1 mb-3">
                            {#each [
                                {color: "#1A56DB", label: "Blue"},
                                {color: "#0E9F6E", label: "Green"},
                                {color: "#FACA15", label: "Yellow"},
                                {color: "#F05252", label: "Red"},
                                {color: "#FF8A4C", label: "Orange"},
                                {color: "#0694A2", label: "Teal"},
                                {color: "#B4C6FC", label: "Light indigo"},
                                {color: "#8DA2FB", label: "Indigo"},
                                {color: "#5145CD", label: "Purple"},
                                {color: "#771D1D", label: "Brown"},
                                {color: "#FCD9BD", label: "Light orange"},
                                {color: "#99154B", label: "Bordo"},
                                {color: "#7E3AF2", label: "Dark Purple"},
                                {color: "#CABFFD", label: "Light"},
                                {color: "#D61F69", label: "Dark Pink"},
                                {color: "#F8B4D9", label: "Pink"},
                                {color: "#F6C196", label: "Cream"},
                                {color: "#A4CAFE", label: "Light Blue"},
                                {color: "#5145CD", label: "Dark Blue"},
                                {color: "#B43403", label: "Orange Brown"},
                                {color: "#FCE96A", label: "Light Yellow"},
                                {color: "#1E429F", label: "Navy Blue"},
                                {color: "#768FFD", label: "Light Purple"},
                                {color: "#BCF0DA", label: "Light Green"},
                                {color: "#EBF5FF", label: "Sky Blue"},
                                {color: "#16BDCA", label: "Cyan"},
                                {color: "#E74694", label: "Pink"},
                                {color: "#83B0ED", label: "Darker Sky Blue"},
                                {color: "#03543F", label: "Forest Green"},
                                {color: "#111928", label: "Black"},
                                {color: "#4B5563", label: "Stone"},
                                {color: "#6B7280", label: "Gray"},
                                {color: "#D1D5DB", label: "Light Gray"},
                                {color: "#F3F4F6", label: "Cloud Gray"},
                                {color: "#F3F4F6", label: "Cloud Gray"},
                                {color: "#F9FAFB", label: "Heaven Gray"},
                            ] as { color, label }}
                                <button type="button" data-hex-color={ color } onclick={ onClickColorHex } style="background-color: { color }" class="w-6 h-6 rounded-md"><span class="sr-only">{ label }</span></button>
                            {/each}
                        </div>
                        <button type="button" onclick={ resetColor } class="py-1.5 text-sm font-medium text-gray-500 focus:outline-none bg-white rounded-lg hover:bg-gray-100 hover:text-gray-900 focus:z-10 focus:ring-4 focus:ring-gray-100 dark:focus:ring-gray-700 dark:bg-gray-700 dark:text-gray-400 dark:hover:text-white w-full dark:hover:bg-gray-600">Reset color</button>
                    </Dropdown>
                    <Tooltip triggeredBy="#toggleTextColorButton">
                        Text size
                    </Tooltip>
                    <button id="toggleFontFamilyButton" type="button" class="custom-btn-toolbar">
                        <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m10.6 19 4.298-10.93a.11.11 0 0 1 .204 0L19.4 19m-8.8 0H9.5m1.1 0h1.65m7.15 0h-1.65m1.65 0h1.1m-7.7-3.985h4.4M3.021 16l1.567-3.985m0 0L7.32 5.07a.11.11 0 0 1 .205 0l2.503 6.945h-5.44Z"/>
                        </svg>
                        <span class="sr-only">Font family</span>
                    </button>
                    <Dropdown bind:open={ fontFamilyDropdownOpen } class="w-72 rounded bg-white p-2 shadow dark:bg-gray-700 text-sm font-medium">
                        {#each [
                            { name: "Default", fontFamily: "Inter, ui-sans-serif", style: "" },
                            { name: "Arial", fontFamily: "Arial, sans-serif", style: "font-family: Arial, sans-serif;" },
                            { name: "Courier New", fontFamily: "'Courier New', monospace", style: "font-family: 'Courier New', monospace;" },
                            { name: "Georgia", fontFamily: "Georgia, serif", style: "font-family: Georgia, serif;" },
                            { name: "Lucida Sans Unicode", fontFamily: "'Lucida Sans Unicode', sans-serif", style: "font-family: 'Lucida Sans Unicode', sans-serif;" },
                            { name: "Tahoma", fontFamily: "Tahoma, sans-serif", style: "font-family: Tahoma, sans-serif;" },
                            { name: "Times New Roman", fontFamily: "'Times New Roman', serif", style: "font-family: 'Times New Roman', serif;" },
                            { name: "Trebuchet MS", fontFamily: "'Trebuchet MS', sans-serif", style: "font-family: 'Trebuchet MS', sans-serif;" },
                            { name: "Verdana", fontFamily: "Verdana, sans-serif", style: "font-family: Verdana, sans-serif;" }
                        ] as { name, fontFamily, style }}
                            <DropdownItem>
                                <button
                                        onclick={onSelectFontFamily}
                                        data-font-family={fontFamily}
                                        type="button"
                                        class="flex justify-between items-center w-full text-sm rounded px-3 py-2 hover:bg-gray-100 text-gray-900 dark:hover:bg-gray-600 dark:text-white"
                                        style={style}
                                >
                                    {name}
                                </button>
                            </DropdownItem>
                        {/each}
                    </Dropdown>
                    <Tooltip triggeredBy="#toggleFontFamilyButton">
                        Font family
                    </Tooltip>
                    <div class="px-1">
                        <span class="block w-px h-4 bg-gray-300 dark:bg-gray-600"></span>
                    </div>
                    <ToolbarButton onclick={() => setTextAlign("left")} tooltipText="Align left">
                        <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 6h8m-8 4h12M6 14h8m-8 4h12"/>
                        </svg>
                    </ToolbarButton>
                    <ToolbarButton onclick={() => setTextAlign("center")} tooltipText="Align center">
                        <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 6h8M6 10h12M8 14h8M6 18h12"/>
                        </svg>
                    </ToolbarButton>
                    <ToolbarButton onclick={() => setTextAlign("right")} tooltipText="Align right">
                        <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 6h-8m8 4H6m12 4h-8m8 4H6"/>
                        </svg>
                    </ToolbarButton>
                </div>
            </div>
            <div class="flex items-center gap-2 pt-2 flex-wrap">
                <button id="typographyDropdownButton" class="flex items-center justify-center rounded-lg bg-gray-100 px-3 py-1.5 text-sm font-medium text-gray-500 hover:bg-gray-200 hover:text-gray-900 focus:z-10 focus:outline-none focus:ring-4 focus:ring-gray-50 dark:bg-gray-600 dark:text-gray-400 dark:hover:bg-gray-500 dark:hover:text-white dark:focus:ring-gray-600" type="button">
                    Format
                    <svg class="-me-0.5 ms-1.5 h-3.5 w-3.5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m19 9-7 7-7-7" />
                    </svg>
                </button>
                <!-- Heading Dropdown -->
                <Dropdown bind:open={ typographyDropdownOpen } class="w-72 rounded bg-white p-2 shadow dark:bg-gray-700 text-sm font-medium">
                    {#each [
                        { label: "Paragraph", keys: ['Cmd', 'Alt', '0'], headingLevel: "0", onclick: onSelectParagraph },
                        { label: "Heading 1", keys: ['Cmd', 'Alt', '1'], headingLevel: "1", onclick: onSelectHeading },
                        { label: "Heading 2", keys: ['Cmd', 'Alt', '2'], headingLevel: "2", onclick: onSelectHeading },
                        { label: "Heading 3", keys: ['Cmd', 'Alt', '3'], headingLevel: "3", onclick: onSelectHeading },
                        { label: "Heading 4", keys: ['Cmd', 'Alt', '4'], headingLevel: "4", onclick: onSelectHeading },
                        { label: "Heading 5", keys: ['Cmd', 'Alt', '5'], headingLevel: "5", onclick: onSelectHeading },
                        { label: "Heading 6", keys: ['Cmd', 'Alt', '6'], headingLevel: "6", onclick: onSelectHeading },
                    ] as { label, keys, headingLevel, onclick }}
                        <button type="button" onclick={onclick} data-heading-level={headingLevel} class="flex justify-between items-center w-full text-base rounded px-3 py-2 hover:bg-gray-100 text-gray-900 dark:hover:bg-gray-600 dark:text-white">{label}
                            <span class="space-x-1.5">
                                {#each keys as key}
                                    <kbd class="px-2 py-1 text-xs font-semibold text-gray-500 bg-gray-100 border border-gray-200 rounded-lg dark:bg-gray-600 dark:text-gray-400 dark:border-gray-500">{key}</kbd>
                                {/each}
                            </span>
                        </button>
                    {/each}
                </Dropdown>
                <div class="ps-1.5">
                    <span class="block w-px h-4 bg-gray-300 dark:bg-gray-600"></span>
                </div>
                <ToolbarButton onclick={addImage} tooltipText="Add image">
                    <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                        <path fill-rule="evenodd" d="M13 10a1 1 0 0 1 1-1h.01a1 1 0 1 1 0 2H14a1 1 0 0 1-1-1Z" clip-rule="evenodd"/>
                        <path fill-rule="evenodd" d="M2 6a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v12c0 .556-.227 1.06-.593 1.422A.999.999 0 0 1 20.5 20H4a2.002 2.002 0 0 1-2-2V6Zm6.892 12 3.833-5.356-3.99-4.322a1 1 0 0 0-1.549.097L4 12.879V6h16v9.95l-3.257-3.619a1 1 0 0 0-1.557.088L11.2 18H8.892Z" clip-rule="evenodd"/>
                    </svg>
                </ToolbarButton>
                <ToolbarButton onclick={addYoutubeVideo} tooltipText="Add Youtube video">
                    <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                        <path fill-rule="evenodd" d="M9 7V2.221a2 2 0 0 0-.5.365L4.586 6.5a2 2 0 0 0-.365.5H9Zm2 0V2h7a2 2 0 0 1 2 2v16a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V9h5a2 2 0 0 0 2-2Zm-2 4a2 2 0 0 0-2 2v2a2 2 0 0 0 2 2h2a2 2 0 0 0 2-2v-2a2 2 0 0 0-2-2H9Zm0 2h2v2H9v-2Zm7.965-.557a1 1 0 0 0-1.692-.72l-1.268 1.218a1 1 0 0 0-.308.721v.733a1 1 0 0 0 .37.776l1.267 1.032a1 1 0 0 0 1.631-.776v-2.984Z" clip-rule="evenodd"/>
                    </svg>
                </ToolbarButton>
                <ToolbarButton onclick={ () => toggleMark('toggleBulletList') } tooltipText="Toggle list">
                    <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                        <path stroke="currentColor" stroke-linecap="round" stroke-width="2" d="M9 8h10M9 12h10M9 16h10M4.99 8H5m-.02 4h.01m0 4H5"/>
                    </svg>
                </ToolbarButton>
                <ToolbarButton onclick={ () => toggleMark('toggleOrderedList') } tooltipText="Toggle ordered list">
                    <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6h8m-8 6h8m-8 6h8M4 16a2 2 0 1 1 3.321 1.5L4 20h5M4 5l2-1v6m-2 0h4"/>
                    </svg>
                </ToolbarButton>
                <ToolbarButton onclick={ () => toggleMark('toggleBlockquote') } tooltipText="Toggle blockquote">
                    <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6h8m-8 6h8m-8 6h8M4 16a2 2 0 1 1 3.321 1.5L4 20h5M4 5l2-1v6m-2 0h4"/>
                    </svg>
                </ToolbarButton>
                <ToolbarButton onclick={ () => toggleMark('setHorizontalRule') } tooltipText="Toggle Horizontal Rule">
                    <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                        <path stroke="currentColor" stroke-linecap="round" stroke-width="2" d="M5 12h14"/>
                        <path stroke="currentColor" stroke-linecap="round" d="M6 9.5h12m-12 9h12M6 7.5h12m-12 9h12M6 5.5h12m-12 9h12"/>
                    </svg>
                </ToolbarButton>
            </div>
        </div>
    </div>
    <div class="px-4 py-2 bg-white rounded-b-lg dark:bg-gray-800">
        <label for="wysiwyg-example" class="sr-only">Publish post</label>
        <div id="wysiwyg-example"
             class="block w-full px-0 text-sm text-gray-800 bg-white border-0 dark:bg-gray-800 focus:ring-0 dark:text-white dark:placeholder-gray-400"></div>
        <EditorContent editor={$editor} />
    </div>
</div>