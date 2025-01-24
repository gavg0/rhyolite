<script lang="ts">
  import type { ChainedCommands } from "@tiptap/core";
  import { Extension } from "@tiptap/core";
  import Bold from "@tiptap/extension-bold";
  import CharacterCount from "@tiptap/extension-character-count";
  import { Color } from "@tiptap/extension-color";
  import Document from "@tiptap/extension-document";
  import FontFamily from "@tiptap/extension-font-family";
  import Highlight from "@tiptap/extension-highlight";
  import Image from "@tiptap/extension-image";
  import Link from "@tiptap/extension-link";
  import TextAlign from "@tiptap/extension-text-align";
  import TextStyle from "@tiptap/extension-text-style";
  import Typography from "@tiptap/extension-typography";
  import Underline from "@tiptap/extension-underline";
  import YouTube from "@tiptap/extension-youtube";
  import StarterKit from "@tiptap/starter-kit";
  import { Dropdown, DropdownItem, Tooltip } from "flowbite-svelte";
  import { onDestroy, onMount } from "svelte";
  import { createEditor, Editor, EditorContent } from "svelte-tiptap";
  import type { Readable } from "svelte/store";
  import CodeBlockLowlight from "@tiptap/extension-code-block-lowlight";
  import ListItem from "@tiptap/extension-list-item";
  import TaskItem from "@tiptap/extension-task-item";
  import TaskList from "@tiptap/extension-task-list";
  import Heading from "@tiptap/extension-heading";
  import { ChevronDownOutline, ChevronUpOutline } from "flowbite-svelte-icons";
  import { all, createLowlight } from "lowlight";
  import ContentEditorStore from "../../stores/content-editor.store";
  import ToolbarButton from "./components/toolbar-button.svelte";

  let editor = $state() as Readable<Editor>;
  const lowlight = createLowlight(all);

  interface ContentEditorProps {
    content: any;
    onchange: (content: Editor) => void;
    class: string;
  }
  const {
    content,
    onchange,
    class: classProp = "",
  }: ContentEditorProps = $props();

  const setupEditor = () => {
    const FontSizeTextStyle = TextStyle.extend({
      addAttributes() {
        return {
          fontSize: {
            default: null,
            renderHTML: (attributes) => {
              if (!attributes.fontSize) {
                return {};
              }
              return { style: `font-size: ${attributes.fontSize}` };
            },
          },
        };
      },
    });

    const CustomHeader = Heading.configure({
      levels: [1, 2, 3, 4, 5, 6],
    }).extend({
      renderHTML({ node, HTMLAttributes }) {
        const level = node.attrs.level as 1 | 2 | 3 | 4 | 5 | 6;
        const sizes = {
          1: "text-5xl text-text font-extrabold my-3",
          2: "text-4xl text-text font-extrabold my-3",
          3: "text-3xl text-text font-extrabold my-3",
          4: "text-2xl text-text font-extrabold my-3",
          5: "text-xl text-text font-extrabold my-3",
          6: "text-lg text-text font-extrabold my-3",
        };

        const className =
          `${HTMLAttributes.class || ""} ${sizes[level]}`.trim();

        return [`h${level}`, { ...HTMLAttributes, class: className }, 0];
      },
    });

    // const CustomBold = Bold.extend({
    //   renderHTML({ HTMLAttributes }) {
    //     const { style, ...rest } = HTMLAttributes;
    //     const newStyle = `font-weight: bold; ${style || ""}`.trim();
    //     return ["span", { ...rest, style: newStyle }, 0];
    //   },
    //   addOptions() {
    //     return {
    //       ...this.parent?.(),
    //       HTMLAttributes: {},
    //     };
    //   },
    // });

    const TabIndent = Extension.create({
      addKeyboardShortcuts() {
        return {
          Tab: () => {
            const { state, dispatch } = this.editor.view;
            const { from, to } = state.selection;
            const indent = "    "; // Four spaces

            // Insert four spaces at the current selection
            dispatch(state.tr.insertText(indent, from, to));
            return true; // Prevents the default behavior
          },
          "Shift-Tab": () => {
            const { state, dispatch } = this.editor.view;
            const { from, to } = state.selection;
            const indent = "    "; // Four spaces
            const startOfLine = state.doc.resolve(from).start();

            // Check if the selection starts with four spaces
            if (state.doc.textBetween(startOfLine, from).startsWith(indent)) {
              // Remove four spaces before the current selection
              dispatch(
                state.tr.delete(startOfLine, startOfLine + indent.length),
              );
            }
            return true; // Prevents the default behavior
          },
        };
      },
    });

    editor = createEditor({
      extensions: [
        StarterKit.configure({
          bold: {
            HTMLAttributes: { class: "text-text font-bold" },
          },
          heading: false,
          blockquote: {
            HTMLAttributes: {
              class:
                "border-l-2 border-subtext2 w-fit px-4 text-text text-lg bg-base/60 font-normal leading-none before:content-none after:content-none",
            },
          },
        }),
        // CustomBold,
        CustomHeader,
        TabIndent,
        TextStyle,
        Color,
        FontSizeTextStyle,
        FontFamily,
        Highlight,
        Underline,
        Typography,
        Document,
        TaskList,
        TaskItem,
        ListItem,
        TaskItem.configure({
          HTMLAttributes: {
            class: "flex gap-2 leading-none", // Added leading-none and changed to items-start
          },
          nested: true,
        }),

        TaskList.configure({
          HTMLAttributes: {
            class: "not-prose pl-2",
          },
        }),
        Link.configure({
          openOnClick: false,
          autolink: true,
          defaultProtocol: "https",
        }),
        TextAlign.configure({
          types: ["heading", "paragraph"],
        }),
        CharacterCount.configure({
          textCounter: (text) => [...new Intl.Segmenter().segment(text)].length,
          wordCounter: (text) =>
            text.split(/\s+/).filter((word) => word !== "").length,
        }),
        CodeBlockLowlight.configure({
          lowlight,
          HTMLAttributes: {
            class: "bg-mantle text-text rounded-lg p-4", // Added bg-surface0
          },
        }),
        Image,
        YouTube,
      ],
      content,
      editorProps: {
        attributes: {
          class:
            "format text-text text-base focus:outline-none format-blue max-w-none leading-1",
        },
      },
      // onTransaction: ({ editor }) => {
      // // force re-render so `editor.isActive` works as expected
      // editor = editor;

      // },
      onCreate: ({ editor }) => {
        // Update counts when editor is initialized
        onchange(editor as Editor);
      },
      onUpdate: ({ editor }) => {
        onchange(editor as Editor);
      },
    });
  };

  onMount(() => {
    setupEditor();
  });

  const toggleMark = (mark: string) => {
    const focused: ChainedCommands = $editor.chain().focus();
    if (typeof (focused as any)[mark] === "function") {
      (focused as any)[mark]().run();
    }
  };

  const toggleLink = () => {
    const url: string | null = window.prompt("Enter image URL:", "https://");
    if (url) $editor.chain().focus().toggleLink({ href: url }).run();
  };

  const addImage = () => {
    const url = window.prompt("Enter image URL:", "https://");
    if (url) {
      $editor.chain().focus().setImage({ src: url }).run();
    }
  };

  const addYoutubeVideo = () => {
    const url = window.prompt(
      "Enter YouTube URL:",
      "https://www.youtube.com/watch?v=",
    );
    if (url) {
      $editor.commands.setYoutubeVideo({
        src: url,
        width: 640,
        height: 480,
      });
    }
  };

  let textSizeDropdownOpen = $state(false);
  const setTextSize = (event: Event) => {
    const fontSize: string | null = (
      event.target as HTMLInputElement
    ).getAttribute("data-text-size");
    // Apply the selected font size via pixels using the TipTap editor chain
    if (fontSize)
      $editor.chain().focus().setMark("textStyle", { fontSize }).run();
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
    const selectedColor: string | null = (
      event.target as HTMLInputElement
    ).getAttribute("data-hex-color");
    // Apply the selected color to the selected text
    if (selectedColor) $editor.chain().focus().setColor(selectedColor).run();
    colorDropdownOpen = false;
  };

  const resetColor = () => {
    $editor.commands.unsetColor();
  };

  let fontFamilyDropdownOpen = $state(false);
  const onSelectFontFamily = (event: Event) => {
    const fontFamily: string | null = (
      event.target as HTMLInputElement
    ).getAttribute("data-font-family");

    // Apply the selected font size via pixels using the TipTap editor chain
    if (fontFamily) $editor.chain().focus().setFontFamily(fontFamily).run();

    // Hide the dropdown after selection
    fontFamilyDropdownOpen = false;
  };

  const setTextAlign = (align: "left" | "right" | "center") => {
    $editor.chain().focus().setTextAlign(align).run();
  };

  let typographyDropdownOpen = $state(false);
  const onSelectHeading = (event: Event) => {
    const headingLevel: string | null = (
      event.target as HTMLInputElement
    ).getAttribute("data-heading-level");
    if (headingLevel) {
      const level = parseInt(headingLevel) as 1 | 2 | 3 | 4 | 5 | 6;
      $editor.chain().focus().toggleHeading({ level }).run();
    }
    typographyDropdownOpen = false;
  };
  const onSelectParagraph = () => {
    $editor.chain().focus().setParagraph().run();
    typographyDropdownOpen = false;
  };

  let flagToolbarVisibility = $state(false);
  const unsubscribeStates = ContentEditorStore.states.subscribe((value) => {
    flagToolbarVisibility = value.flagToolbarVisibility;
  });
  onDestroy(unsubscribeStates); // Clean up
</script>

<div class={`flex flex-col h-full w-full text-text ${classProp}`}>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="grow px-4 py-2 rounded-b-lg"
    onclick={() => $editor.commands.focus()}
    onfocus={() => $editor.commands.focus()}
    role="textbox"
    tabindex="-1"
  >
    <EditorContent class="cursor-text h-full text-text" editor={$editor} />
  </div>
</div>
