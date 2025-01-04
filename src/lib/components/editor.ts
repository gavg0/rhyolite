import type { Editor } from '@tiptap/core';

interface EditorStore {
  instance: Editor | null;
}

export const editorStore = $state<EditorStore>({ instance: null });