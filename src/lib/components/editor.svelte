<script lang="ts">
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
  import { autoSave, loadRecentDocuments } from '../functions/functions.svelte';  
  import { editorStore } from '../components/editor';
  import {
            updateTabs,
            addnewtab,
            switchTab,
            getTabs,
            cycleTabs,
            gotoLastTab,
            gotoTab1,
            returnTabsArray
        }
        from "../components/tabsbar.svelte";
  import {
            updateTitleText,
            returnTitleText
        }
        from "../components/titlebox.svelte";

  import { updateWordCount, updateCharCount } from "../components/word-char-counter.svelte";

  let editor: Editor;
  let element: Element;
  

  setContext(
        'editor',
        {
            setEditorContent,
            getEditorContent,
            getEditorContentasText
        }
    );

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

    editorStore.set(editor);

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
      editorStore.set(null);
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
    let characterCount = editor.storage.characterCount.characters();
    let wordsCount = editor.storage.characterCount.words();

    updateCharCount(characterCount);
    updateWordCount(wordsCount);
  }
</script>

<main class="h-full overflow-hidden">
  <div class="flex rounded-lg  m-[0.5%] w-[70%] h-full flex-grow cursor-text mx-auto">
    <div class="w-full h-full" bind:this={element}></div>
  </div>
</main>
