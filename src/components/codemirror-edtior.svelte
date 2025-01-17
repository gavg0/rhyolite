<script lang="ts">
  import { EditorState } from "@codemirror/state";
  import { EditorView, keymap } from "@codemirror/view";
  import { defaultKeymap } from "@codemirror/commands";
  import { minimalSetup } from "codemirror";
  import { markdown, commonmarkLanguage } from "@codemirror/lang-markdown";
  import { languages } from "@codemirror/language-data";
  import { autocompletion } from "@codemirror/autocomplete";
  import { onMount } from "svelte";
  import { marked } from "marked";

  let editorContainer: HTMLElement;
  let view: EditorView;

  const initial_text = "Welcome to Rhyolite!";

  const customTheme = EditorView.theme({
    ".cm-cursor": { borderLeftColor: "rgb(var(--color-text))" },
    "&.cm-focused .cm-cursor": { borderLeftColor: "rgb(var(--color-text))" },
  });

  onMount(() => {
    view = new EditorView({
      doc: initial_text,
      extensions: [
        minimalSetup,
        markdown({
          codeLanguages: languages,
          base: commonmarkLanguage,
          completeHTMLTags: true,
          addKeymap: true,
        }),
        EditorView.lineWrapping,
        customTheme,
        // autocompletion(),
      ],
      parent: editorContainer,
    });

    return () => {
      view.destroy();
    };
  });
</script>

<main>
  <div bind:this={editorContainer} class="w-full h-full m-5 text-text"></div>
</main>
