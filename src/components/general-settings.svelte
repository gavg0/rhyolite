<script lang="ts">
  import { onMount } from "svelte";
  import GeneralSettingsStore from "../stores/general-settings.store";
  import generalSettingsStore from "../stores/general-settings.store";
  import { invoke } from '@tauri-apps/api/core';

  interface Single_setting {
  title: string;
  description: string;
  select?: string[];
  selected?: string;
  check?: boolean;
}

  interface Setting {
      title: string;
      description: string;
      setting?: Single_setting[];
    }
const updateSetting = async (title: string, value: string) => {
  try {
    await invoke('save_setting', { 
      title: title,
      value: value 
    });
    console.log('Setting updated:', title, value);
    await get_settings(); // Refresh settings after save
  } catch (error) {
    console.error('Failed to update setting:', error);
  }
};

const toggle_check = async (title: string) => {
  try {
    await invoke('toggle_check', { title });
    console.log('Setting updated:', title);
    await get_settings(); // Refresh settings after save
  } catch (error) {
    console.error('Failed to update setting:', error);
  }
};
const get_settings = async () => {  
  try {
    settings = await invoke('get_all_settings');
    console.log('UserSettings received:', settings);
  } catch (error) {
    console.error('Failed to get settings:', error);
  }
};

const get_selected_setting = async (title: string) => {  
  try {
  const setting = await invoke('get_setting_value', { title });
    console.log('Setting received:', setting);
    return setting;
  } catch (error) {
    console.error('Failed to get setting:', error);
  }
};

let isVisible: boolean = $state(false);
let searchText: string = $state("");
let selectedTab: number = $state(0);  
let settings: Setting[] = $state([]);             // These are the settings the user currently has


  const handleKeydown = (e: KeyboardEvent) => {
    if (e.key === "Enter") {
      console.log(searchText);
    }
  };


  GeneralSettingsStore.states.subscribe((state) => {
    isVisible = state.flagGeneralSettingsVisibility;
  });

  onMount(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === "Escape" && isVisible) {
        GeneralSettingsStore.toggleVisibility();
      }
    };
    document.addEventListener("keydown", handleEscape);
    get_settings();
    return () => {
      document.removeEventListener("keydown", handleEscape);
    };
  });

</script>

<!-- <svelte:window on:keydown={handleKeydown} /> -->
{#if isVisible}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="fixed top-0 left-0 w-full h-full bg-black/60 z-20"
    aria-modal="true"
    role="dialog"
    onclick={(e) => {
      if (e.target === e.currentTarget) GeneralSettingsStore.toggleVisibility();
    }}
    onkeydown={(e) => {
      if (e.key === "Escape") GeneralSettingsStore.toggleVisibility();
    }}
  >
    <div
      class="fixed flex flex-col text-text bg-crust rounded-lg w-min-[200px] w-[80%] max-w-[800px] h-[80vh] left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 overflow-auto"
    >
      <div class="flex flex-row h-full">
        <div
          class="h-full w-1/4 max-w-[500px] min-w-[150px] bg-base pt-1.5 px-1.5"
        >
            <div class="flex flex-col gap-1.5">
                {#each settings as setting, index}
                <button
                    class="text-left w-full text-md rounded-md p-1.5 transition duration-100 
                    {selectedTab === index ? 'bg-surface0' : 'hover:bg-surface0'}"
                    onclick={() => (selectedTab = index)}>{setting.title}</button>
                {/each}
            </div>
        </div>
        <div class="h-full overflow-y-auto w-full mx-auto pt-2 px-2">
            {#each settings as setting, index}
              {#if index === selectedTab}
                <div class="flex flex-col gap-2">
                    <div class="flex flex-row justify-between">
                        <h2 class="text-2xl font-semibold">{setting.title}</h2>
                        <button onclick={() => (generalSettingsStore.toggleVisibility())} aria-label="Close settings" class="text-text transition duration-200 rounded-lg hover:bg-surface2 w-6 h-6"><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-x"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg></button>
                    </div>
                  </div>
                    {#if setting.setting}
                    {#each setting.setting as subsetting, index}
                    <div class="border-b border-surface2 w-full max-w-[500px] rounded-xl drop-shadow-xl {index != 0 ? 'my-3' : 'mb-2'}"></div>
                        <div class="flex md:flex-row max-w-[500px] flex-col md:justify-between gap-1">
                            <div>
                                <h3 class="text-lg font-semibold">{subsetting.title}</h3>
                                <p class="text-sm">{subsetting.description}</p>
                            </div>
                            {#if subsetting.select}
                            <select 
                                class="w-full shrink-0 md:w-[200px] h-[40px] p-1 rounded-md border border-surface2 hover:border-blue-500 transition duration-150 bg-base text-text"
                                onchange={(e) => {
                                    const value = (e.target as HTMLSelectElement).value;
                                    updateSetting(subsetting.title, value);
                                }}
                            >
                                {#each subsetting.select as option}
                                  <option value={option} selected={option === subsetting.selected}>{option}</option>
                                {/each}
                            </select>
                            {:else if subsetting.check !== undefined}
                            <input 
                                type="checkbox" 
                                checked={subsetting.check}
                                onchange={() => {
                                    toggle_check(subsetting.title);
                                }}
                                class="w-6 h-6 rounded-md shadow-2xl bg-base"
                            />
                            {/if} 
                        </div>
                    {/each}
                    {/if}
                    

                    {/if}
            {/each}
            <button onclick={() => (get_settings())} class="w-full p-2 rounded-lg bg-surface2 text-text hover:bg-surface1 transition duration-200">Close</button>
        </div>
      </div>
    </div>
  </div>
{/if}
