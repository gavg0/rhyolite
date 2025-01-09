import { type Writable, writable, get } from 'svelte/store';

interface IContentEditorStore {
    flagToolbarVisibility: boolean;
}

const contentEditorStore: Writable<IContentEditorStore> = writable<IContentEditorStore>({
    flagToolbarVisibility: false,
});

const isToolbarVisible = (): boolean => {
    const { flagToolbarVisibility }: IContentEditorStore = get(contentEditorStore);
    return flagToolbarVisibility;
}

const toggleToolbarVisibility = (): boolean => {
    const { flagToolbarVisibility }: IContentEditorStore = get(contentEditorStore);
    contentEditorStore.update(() => ({
        flagToolbarVisibility: !flagToolbarVisibility,
    }));
    return !flagToolbarVisibility;
}

export default {
    contentEditorStore,
    isToolbarVisible,
    toggleToolbarVisibility,
}