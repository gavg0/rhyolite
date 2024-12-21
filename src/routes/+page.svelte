<script lang="ts">
import { onMount } from 'svelte';
import { invoke } from "@tauri-apps/api/core";
import '../styles/styles.css';
import { v4 as uuidv4 } from 'uuid';
import Quill from 'quill';

interface Tab {
  id: string;
  title: string;
  order: number;
}

interface Document {
  id: string;
  title: string;
  content: string;
}

interface QuickFormatOption {
  icon: string;
  format: string;
  value?: any;
}

interface ToolbarOptions {
  [key: string]: any;
}

// State management
let titleText: string = $state('');
let textboxText: string = '';
let recentDocuments: Document[] = [];
let currentId: string = $state('');
let wordCount: number = $state(0);
let charCount: number = $state(0);
let isToolbarVisible: boolean = $state(false);
let tabCount: number = $state(1);
let tabs: Tab[] = $state([]);

// Initialize Quill
let quill: Quill;
const toolbarOptions: any[] = [
  ['bold', 'italic', 'underline', 'strike'],
  ['blockquote', 'code-block'],
  [{ header: 1 }, { header: 2 }],
  [{ list: 'ordered' }, { list: 'bullet' }],
  [{ script: 'sub' }, { script: 'super' }],
  [{ indent: '-1' }, { indent: '+1' }],
  [{ direction: 'rtl' }],
  [{ size: ['small', false, 'large', 'huge'] }],
  [{ header: [1, 2, 3, 4, 5, 6, false] }],
  [{ color: [] }, { background: [] }],
  [{ font: [] }],
  [{ align: [] }],
  ['clean'],
  ['link', 'image', 'video'],
];

let customTooltip: HTMLDivElement;
const quickFormatOptions: QuickFormatOption[] = [
  { icon: 'B', format: 'bold' },
  { icon: 'I', format: 'italic' },
  { icon: 'U', format: 'underline' },
  { icon: 'S', format: 'strike' },
  { icon: '¶', format: 'blockquote' },
  { icon: '<>', format: 'code-block' }
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

  const editorElement = document.querySelector('#editor');
  const toolbarElement = document.querySelector('.ql-toolbar');
  
  if (editorElement) {
    editorElement.classList.add('quill-dark-theme');
  }
  
  if (toolbarElement instanceof HTMLElement) {
    toolbarElement.style.display = 'none';
  }

  // Create custom tooltip element
  customTooltip = document.createElement('div');
  customTooltip.className = 'ql-tooltip';
  document.body.appendChild(customTooltip);

  // Add selection change handler
  quill.on('selection-change', (range: { index: number; length: number } | null) => {
    if (range && range.length > 0) {
      const bounds = quill.getBounds(range.index, range.length);
      const quillContainer = document.querySelector('#editor');
      
      if (quillContainer instanceof HTMLElement) {
        const containerBounds = quillContainer.getBoundingClientRect();

        customTooltip.style.position = 'absolute';
        customTooltip.style.left = `${containerBounds.left + bounds.left}px`;
        customTooltip.style.top = `${containerBounds.top + bounds.top - 40}px`;
        
        // Clear existing buttons
        customTooltip.innerHTML = '';
        
        // Add format buttons
        quickFormatOptions.forEach(option => {
          const button = document.createElement('button');
          button.textContent = option.icon;
          button.onclick = () => {
            if (option.value !== undefined) {
              quill.format(option.format, option.value);
            } else {
              quill.format(option.format, !quill.getFormat()[option.format]);
            }
          };
          customTooltip.appendChild(button);
        });

        customTooltip.style.display = 'flex';
      }
    } else {
      customTooltip.style.display = 'none';
    }
  });

  quill.on('text-change', () => {
    const text = quill.getText() || '';
    wordCount = countWords(text);
    charCount = Math.max(0, text.length - 1);
  });

  const toolbar = document.querySelector('.ql-toolbar');
  if (toolbar instanceof HTMLElement) {
    toolbar.style.display = 'none';
    toolbar.classList.remove('visible');
  }
    
  loadRecentDocuments().then(() => {
    if (tabs.length === 0) {
      addnewtab();
    }
  });

  // Set up auto-save
  const autoSaveInterval = setInterval(autoSave, 500);

  return () => {
    // Cleanup
    customTooltip?.remove();
    clearInterval(autoSaveInterval);
  };
});

function countWords(text: string): number {
  return text.split(/\s+/).filter(word => word.length > 0).length;
}

async function addnewtab(): Promise<void> {
  const newTab: Tab = await invoke('new_tab');
  tabs = [newTab];
  currentId = newTab.id;
  titleText = newTab.title;
}

async function switchTab(tabId: string): Promise<void> {
  try {
    const docResult: Document | null = await invoke('get_document_content', { id: tabId });
    
    if (docResult) {
      currentId = tabId;
      titleText = docResult.title;
      quill?.setContents(JSON.parse(docResult.content));
    } else {
      currentId = tabId;
      titleText = '';
      quill?.setContents([]);
    }
  } catch (error) {
    console.error('Failed to switch tab:', error);
  }
}

async function cycleTabs(): Promise<void> {
  if (tabs.length > 0) {
    const currentTabIndex = tabs.findIndex(tab => tab.id === currentId);
    const nextTabIndex = (currentTabIndex + 1) % tabs.length;
    const nextTab = tabs[nextTabIndex];
    await switchTab(nextTab.id);
  }
}

async function autoSave(): Promise<void> {
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

async function loadRecentDocuments(): Promise<void> {
  try {
    const docs: Document[] = await invoke('load_recent_files');
    recentDocuments = docs;
    
    if (recentDocuments.length > 0) {
      await invoke('reset_tab_order_count');
      
      for (const doc of recentDocuments) {
        const newTab: Tab = await invoke('load_tab', {
          idIn: doc.id,
          title: doc.title
        });
        tabs = [...tabs, newTab];
      }
      
      const lastDoc = recentDocuments[recentDocuments.length - 1];
      currentId = lastDoc.id;
      titleText = lastDoc.title;
      quill?.setContents(JSON.parse(lastDoc.content));
    }
  } catch (error) {
    console.error('Failed to load documents:', error);
  }
}

function handleTitleChange(event: Event): void {
  const target = event.target as HTMLTextAreaElement;
  titleText = target.value;
}

function handleKeydown(event: KeyboardEvent): void {
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
  if (event.ctrlKey && event.key === 'Tab') {
    event.preventDefault();
    cycleTabs();
  }
}

function toggleToolbar(): void {
  isToolbarVisible = !isToolbarVisible;
  const toolbar = document.querySelector('.ql-toolbar');
  if (toolbar) {
    if (isToolbarVisible) {
      toolbar.style.display = 'block';
      setTimeout(() => {
        toolbar.classList.add('visible');
      }, 10);
    } else {
      toolbar.classList.remove('visible');
      setTimeout(() => {
        toolbar.style.display = 'none';
      }, 300);
    }
  }
}

async function deleteDocument(): Promise<void> {
  try {
    await invoke('delete_document', { id: currentId });
    
    const reorderedTabs: Tab[] = await invoke('reorder_tabs');
    tabs = reorderedTabs;
    
    if (tabs.length > 0) {
      const lastTab = tabs[tabs.length - 1];
      currentId = lastTab.id;
      const docResult: Document = await invoke('get_document_content', { id: currentId });
      titleText = docResult.title;
      quill?.setContents(JSON.parse(docResult.content));
    } else {
      await invoke('reset_tab_order_count')
      await newDocument();
    }
  } catch (error) {
    console.error('Failed to delete document:', error);
  }
}

async function newDocument(): Promise<void> {
  try {
    const newTab: Tab = await invoke('new_tab');
    tabs = [...tabs, newTab];
    currentId = newTab.id;
    titleText = newTab.title;
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

<div id="editor" class="quillbox-container">
  <div class="ql-toolbar ql-snow" class:visible={isToolbarVisible}></div>
  <div class="ql-container ql-snow"></div>
</div>

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
    >
      {tab.order}
    </button>
  {/each}
</div>
</main>
