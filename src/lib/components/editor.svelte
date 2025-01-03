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
  import { Color } from '@tiptap/extension-color';
  import Bold from '@tiptap/extension-bold';
  
  let editor: Editor;
  let element: Element;
  let io: any = getContext('io');
  let tabs: any = getContext('tabs');
  let title: any = getContext('title');

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
        Image,
        YouTube
      ],
      content: `<p>Flowbite is an <strong>open-source library of UI components</strong>...</p>`,
      editorProps: {
        attributes: {
          class: 'format lg:format-lg text-text focus:outline-none format-blue max-w-none'
        }
      }
    });

    io.loadRecentDocuments().then(async () => {
            await tabs.updateTabs();
            let currentTabs = tabs.returnTabsArray();
            if (currentTabs.length === 0) {
                await tabs.addnewtab();
            }
        });
    
    // Set up auto-save
    const autoSaveInterval = setInterval(io.autoSave, 500);
  
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
</script>

<div class="flex flex-col items-center rounded-lg mb-[45px] m-[0.5%] cursor-text">
  <div class="w-[70%] h-full" bind:this={element}></div>
</div>