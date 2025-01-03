<script lang="ts">
    
</script>

<div class="tab-counter" role="tablist" aria-label="Document tabs">
    {#each currentTabs as tab}
        <button
            type="button"
            class="tab-square"
            class:active={currentId === tab.id}
            role="tab"
            aria-selected={currentId === tab.id}
            aria-controls="editor"
            onclick={() => switchTab(tab.id)}
        >
            {tab.title.length > 10 ? tab.title.slice(0, 15) + '...' : tab.title || 'Untitled'}
        </button>
    {/each}
</div>