<script>
  // @ts-nocheck
  import { onMount } from 'svelte';
  import { invoke } from "@tauri-apps/api/core";
  import '../styles/styles.css';
  import '../styles/quill-dark-theme.css';
  import { v4 as uuidv4 } from 'uuid';
  import Quill from 'quill';

  // State management
  let titleText = $state('');
  let textboxText = '';
  let recentDocuments = [];
  let currentId = $state('');
  let wordCount = $state(0);
  let charCount = $state(0);
  let isToolbarVisible = $state(false);
  let tabCount = $state(1);
  let tabs = $state([]);

  // Initialize Quill
  let quill;
  const toolbarOptions = [
          ['bold', 'italic', 'underline', 'strike'], // Formatting buttons
          ['blockquote', 'code-block'], // Block types

          [{ header: 1 }, { header: 2 }], // Header options
          [{ list: 'ordered' }, { list: 'bullet' }], // List options
          [{ script: 'sub' }, { script: 'super' }], // Subscript/Superscript
          [{ indent: '-1' }, { indent: '+1' }], // Indent
          [{ direction: 'rtl' }], // Text direction

          [{ size: ['small', false, 'large', 'huge'] }], // Font size
          [{ header: [1, 2, 3, 4, 5, 6, false] }], // Header levels

          [{ color: [] }, { background: [] }], // Text color and background
          [{ font: [] }], // Font family
          [{ align: [] }], // Text alignment

          ['clean'], // Remove formatting
          ['link', 'image', 'video'], // Media
        ];

  onMount(() => {
    // Initialize Quill editor
    quill = new Quill('#editor', {
      theme: 'snow',
      placeholder: 'start typing...',
      modules: {
            toolbar: toolbarOptions,
          },
      bounds: '#editor'
    });

    document.querySelector('#editor').classList.add('quill-dark-theme');
    document.querySelector('.ql-toolbar').style.display = 'none';

    quill.on('text-change', () => {
        const text = quill.getText() || '';
        wordCount = countWords(text);
        charCount = Math.max(0, text.length - 1);
    });
    
    loadRecentDocuments();
    
    // If no documents were loaded, create a new tab
    if (recentDocuments.length === 0) {
      addnewtab()
    }

    // Set up auto-save
    const autoSaveInterval = setInterval(autoSave, 1000);

    return () => {
      clearInterval(autoSaveInterval);
    };
  });

  function countWords(text) {
    return text.split(/\s+/).filter(word => word.length > 0).length;
  }

  async function addnewtab() {
    const newTab = await invoke('new_tab');
      tabs = [newTab];
      currentId = newTab.id;
  }

  async function switchTab(tabId) {
    try {
        const docResult = await invoke('get_document_content', { id: tabId });
        
        if (docResult) {
            // Document exists, load its content
            currentId = tabId;
            titleText = docResult.title;
            quill?.setContents(JSON.parse(docResult.content));
        } else {
            // No document exists for this tab (new tab)
            currentId = tabId;
            titleText = '';
            quill?.setContents([]);
        }
    } catch (error) {
        console.error('Failed to switch tab:', error);
    }
}

  async function autoSave() {
    if (!titleText && !quill?.getText().trim()) return;

    try {
      await invoke('save_document', {
        id: currentId,
        title: titleText,
        content: JSON.stringify(quill.getContents())
      });
    } catch (error) {
      console.error('Auto-save failed:', error);
    }
  }

  async function loadRecentDocuments() {
    try {
      const docs = await invoke('load_recent_files');
      recentDocuments = docs;
      
      if (recentDocuments.length > 0) {
        // Create tabs for each loaded document
        tabs = recentDocuments.map(doc => ({
          order: tabs.length + 1,
          id: doc.id,
          title: doc.title
        }));
        
        // Load the last document
        const lastDoc = recentDocuments[recentDocuments.length - 1];
        currentId = lastDoc.id;
        titleText = lastDoc.title;
        quill?.setContents(JSON.parse(lastDoc.content));
      }
    } catch (error) {
      console.error('Failed to load documents:', error);
    }
  }

  function handleTitleChange(event) {
    titleText = event.target.value;
  }

  function handleKeydown(event) {
    if (event.ctrlKey && event.key === 'd') {
      event.preventDefault();
      deleteDocument();
    }
    if (event.ctrlKey && event.key === 'n') {
      event.preventDefault();
      newDocument();
    }
    if (event.ctrlKey && event.key === 't') {
      event.preventDefault();
      toggleToolbar();
    }
  }

  // New function to toggle toolbar
  function toggleToolbar() {
    isToolbarVisible = !isToolbarVisible;
    const toolbar = document.querySelector('.ql-toolbar');
    if (toolbar) {
      toolbar.style.display = isToolbarVisible ? 'block' : 'none';
    }
  }

  async function deleteDocument() {
    try {
      await invoke('delete_document', { id: currentId });
      tabs = tabs.filter(tab => tab.id !== currentId);
      
      if (tabs.length > 0) {
        // Switch to the last remaining tab
        const lastTab = tabs[tabs.length - 1];
        currentId = lastTab.id;
        const docResult = await invoke('get_document_content', { id: currentId });
        titleText = docResult.title;
        quill?.setContents(JSON.parse(docResult.content));
      } else {
        // If no tabs remain, create a new one
        await invoke('reset_tab_order_count')
        await newDocument();
      }
    } catch (error) {
      console.error('Failed to delete document:', error);
    }
  }

  async function newDocument() {
    try {
      const newTab = await invoke('new_tab');
      tabs = [...tabs, newTab];
      currentId = newTab.id;
      titleText = '';
      quill?.setContents([]);
    } catch (error) {
      console.error('Failed to create new document:', error);
    }
  }

  // Update word and character counts when text changes
  $effect(() => {
      if (quill) {
          const text = quill.getText() || '';
          wordCount = countWords(text);
          charCount = Math.max(0, text.length - 1);
      }
  });
</script>

<svelte:window on:keydown={handleKeydown}/>

<main class="container">
  <div class="title-container title-textarea">
    <textarea
      class="rounded-container"
      placeholder="Enter Title here..."
      value={titleText}
      oninput={handleTitleChange}
    ></textarea>
  </div>
  
  <div id="editor" class="quillbox-container"></div>
  
  <div class="word-char-counter">
    {wordCount} Words {charCount} Characters
  </div>

  <div class="tab-counter" role="tablist" aria-label="Document tabs">
    {#each tabs as tab}
      <button
        type="button"
        class="tab-square"
        class:active={currentId === tab.id}
        role="tab"
        aria-selected={currentId === tab.id}
        aria-controls="editor"
        onclick={() => switchTab(tab.id)}
        onkeydown={(e) => {
          if (e.key === 'Enter' || e.key === ' ') {
            e.preventDefault();
            switchTab(tab.id);
          }
        }}
      >
        {tab.order}
      </button>
    {/each}
  </div>
</main>
