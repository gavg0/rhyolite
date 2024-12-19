<script>
  // @ts-nocheck
  import { onMount } from 'svelte';
  import { invoke } from "@tauri-apps/api/core";
  import '../styles/styles.css';
  import { v4 as uuidv4 } from 'uuid';
  import Quill from 'quill';

  // State management
  let titleText = $state('');
  let textboxText = '';
  let recentDocuments = [];
  let currentId = $state('');
  let wordCount = $state(0);
  let charCount = $state(0);
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
    });

    quill.on('text-change', () => {
        const text = quill.getText() || '';
        wordCount = countWords(text);
        charCount = Math.max(0, text.length - 1);
    });
    
    // Generate new ID if none exists
    if (!currentId) {
      currentId = uuidv4();
    }
    
    // Load recent documents
    loadRecentDocuments();

    // Set up auto-save
    const autoSaveInterval = setInterval(autoSave, 1000);

    return () => {
      clearInterval(autoSaveInterval);
    };
  });

  function countWords(text) {
    return text.split(/\s+/).filter(word => word.length > 0).length;
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
    if (event.ctrlKey && event.key === 'x') {
      event.preventDefault();
      deleteDocument();
    }
  }

  async function deleteDocument() {
    try {
      await invoke('delete_document', { id: currentId });
      currentId = '';
      titleText = '';
      quill?.setContents([]);
    } catch (error) {
      console.error('Failed to delete document:', error);
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
    Words: {wordCount} | Characters: {charCount}
  </div>
</main>
