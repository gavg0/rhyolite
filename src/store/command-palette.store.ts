import { type Writable, writable, get } from 'svelte/store';

interface ICommandPaletteStore {
    flagCommandPaletteVisibility: boolean;
}

const commandPaletteStore: Writable<ICommandPaletteStore> = writable<ICommandPaletteStore>({
    flagCommandPaletteVisibility: false,
});

const isVisible = (): boolean => {
    const { flagCommandPaletteVisibility }: ICommandPaletteStore = get(commandPaletteStore);
    return flagCommandPaletteVisibility;
}

const toggleVisibility = (): boolean => {
    const { flagCommandPaletteVisibility }: ICommandPaletteStore = get(commandPaletteStore);
    commandPaletteStore.update(() => ({
        flagCommandPaletteVisibility: !flagCommandPaletteVisibility,
    }));
    console.log(!flagCommandPaletteVisibility);
    return !flagCommandPaletteVisibility;
}

export default {
    commandPaletteStore,
    isVisible,
    toggleVisibility,
}