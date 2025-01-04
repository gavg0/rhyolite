<script lang="ts" module>
    import { get } from 'svelte/store';
    import { editorStore } from '../components/editor';
    import type { Editor } from '@tiptap/core';
    import CharacterCount from '@tiptap/extension-character-count'

    let wordCount: number = $state(0);
    let charCount: number = $state(0);

    export function updateWordCount(count: number) {
        wordCount = count;
    }

    export function updateCharCount(count: number) {
        charCount = count;
    }
</script>

<div class="fixed flex flex-row gap-[20px] self-end bottom-[10px] right-[10px] bg-base px-[10px] py-[5px] rounded-[18px] z-10 text-text text-[0.85em]">
    <div>{wordCount} Words</div>
    <div>{charCount} Characters</div>
</div>