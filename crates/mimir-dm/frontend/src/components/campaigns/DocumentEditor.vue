<template>
  <div class="document-editor">
    <!-- Editor Header -->
    <div class="editor-header">
      <div class="header-left">
        <button class="btn-icon" @click="$emit('close')" title="Back to overview">
          ← Back
        </button>
        <h2>{{ document?.title || 'Untitled Document' }}</h2>
      </div>
      <div class="header-right">
        <span v-if="saveStatus" class="save-status" :class="saveStatus">
          {{ saveStatusText }}
        </span>
        <button 
          class="btn-secondary"
          @click="togglePreview"
          :class="{ active: showPreview }"
        >
          {{ showPreview ? 'Edit' : 'Preview' }}
        </button>
        <button 
          class="btn-primary"
          @click="markComplete"
          :disabled="document?.completed_at"
        >
          {{ document?.completed_at ? 'Completed' : 'Mark Complete' }}
        </button>
      </div>
    </div>

    <!-- Editor Content -->
    <div class="editor-content">
      <!-- Editor -->
      <div class="editor-wrapper">
        <!-- Only show toolbar when not in preview mode -->
        <div v-if="editor && !showPreview" class="editor-toolbar">
          <button
            @click="editor?.chain().focus().toggleHeading({ level: 1 }).run()"
            :class="{ 'is-active': editor?.isActive('heading', { level: 1 }) }"
            class="toolbar-btn"
          >
            H1
          </button>
          <button
            @click="editor?.chain().focus().toggleHeading({ level: 2 }).run()"
            :class="{ 'is-active': editor?.isActive('heading', { level: 2 }) }"
            class="toolbar-btn"
          >
            H2
          </button>
          <button
            @click="editor?.chain().focus().toggleHeading({ level: 3 }).run()"
            :class="{ 'is-active': editor?.isActive('heading', { level: 3 }) }"
            class="toolbar-btn"
          >
            H3
          </button>
          <div class="toolbar-divider"></div>
          <button
            @click="editor?.chain().focus().toggleBold().run()"
            :class="{ 'is-active': editor?.isActive('bold') }"
            class="toolbar-btn"
          >
            <strong>B</strong>
          </button>
          <button
            @click="editor?.chain().focus().toggleItalic().run()"
            :class="{ 'is-active': editor?.isActive('italic') }"
            class="toolbar-btn"
          >
            <em>I</em>
          </button>
          <button
            @click="editor?.chain().focus().toggleStrike().run()"
            :class="{ 'is-active': editor?.isActive('strike') }"
            class="toolbar-btn"
          >
            <strike>S</strike>
          </button>
          <div class="toolbar-divider"></div>
          <button
            @click="editor?.chain().focus().toggleBulletList().run()"
            :class="{ 'is-active': editor?.isActive('bulletList') }"
            class="toolbar-btn"
          >
            • List
          </button>
          <button
            @click="editor?.chain().focus().toggleOrderedList().run()"
            :class="{ 'is-active': editor?.isActive('orderedList') }"
            class="toolbar-btn"
          >
            1. List
          </button>
          <button
            @click="editor?.chain().focus().toggleBlockquote().run()"
            :class="{ 'is-active': editor?.isActive('blockquote') }"
            class="toolbar-btn"
          >
            " Quote
          </button>
          <div class="toolbar-divider"></div>
          <button
            @click="editor?.chain().focus().setHorizontalRule().run()"
            class="toolbar-btn"
          >
            — Rule
          </button>
          <button
            @click="editor?.chain().focus().undo().run()"
            :disabled="!editor?.can().undo()"
            class="toolbar-btn"
          >
            ↶ Undo
          </button>
          <button
            @click="editor?.chain().focus().redo().run()"
            :disabled="!editor?.can().redo()"
            class="toolbar-btn"
          >
            ↷ Redo
          </button>
        </div>
        <EditorContent :editor="editor" class="editor-area" :class="{ 'preview-mode': showPreview }" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch, computed, nextTick } from 'vue'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import Placeholder from '@tiptap/extension-placeholder'
import { Markdown } from 'tiptap-markdown-3'
import { invoke } from '@tauri-apps/api/core'
import { debounce } from '../../utils/debounce'

const props = defineProps<{
  document: any
  campaignId: number
}>()

const emit = defineEmits<{
  close: []
  updated: [document: any]
}>()

// State
const showPreview = ref(false)
const saveStatus = ref<'saving' | 'saved' | 'error' | null>(null)
const pendingContent = ref<string | null>(null)

// Initialize Tiptap editor with markdown support
const editor = useEditor({
  content: '',
  extensions: [
    StarterKit.configure({
      heading: {
        levels: [1, 2, 3, 4, 5, 6]
      }
    }),
    Placeholder.configure({
      placeholder: 'Start writing your document...'
    }),
    Markdown.configure({
      html: true,
      tightLists: true,
      tightListClass: 'tight',
      bulletListMarker: '-',
      linkify: false,
      breaks: false
    })
  ],
  onCreate: ({ editor: e }) => {
    // Load document when editor is ready
    if (props.document && pendingContent.value === null) {
      // Use a small delay to ensure editor is fully ready
      setTimeout(() => loadDocument(), 50)
    }
  },
  onUpdate: ({ editor }) => {
    debouncedSave()
  }
})

// Computed
const saveStatusText = computed(() => {
  switch (saveStatus.value) {
    case 'saving': return 'Saving...'
    case 'saved': return 'Saved'
    case 'error': return 'Error saving'
    default: return ''
  }
})

// Load document content
const loadDocument = async () => {
  if (!props.document?.file_path) return
  
  try {
    const response = await invoke<{ data: string }>('read_document_file', {
      filePath: props.document.file_path
    })
    
    if (response.data) {
      // Set markdown content - Tiptap will parse it
      if (editor.value) {
        editor.value.commands.setContent(response.data)
      } else {
        // Store content to set later
        pendingContent.value = response.data
      }
    }
  } catch (e) {
    console.error('Failed to load document:', e)
  }
}

// Get content as markdown
const getMarkdown = (): string => {
  if (!editor.value) return ''
  
  // Use the markdown extension to get markdown content
  // TypeScript workaround for the storage type
  const storage = editor.value.storage as any
  return storage.markdown?.getMarkdown() || ''
}

// Save document content
const saveDocument = async () => {
  if (!props.document?.file_path) return
  
  saveStatus.value = 'saving'
  
  try {
    // Get content as markdown
    const markdown = getMarkdown()
    
    await invoke('save_document_file', {
      filePath: props.document.file_path,
      content: markdown
    })
    
    // Update document in database
    await invoke('update_document', {
      documentId: props.document.id,
      update: {
        updated_at: new Date().toISOString()
      }
    })
    
    saveStatus.value = 'saved'
    setTimeout(() => {
      saveStatus.value = null
    }, 2000)
  } catch (e) {
    console.error('Failed to save document:', e)
    saveStatus.value = 'error'
  }
}

// Debounced save function
const debouncedSave = debounce(saveDocument, 1000)



// Toggle preview mode
const togglePreview = () => {
  showPreview.value = !showPreview.value
  if (editor.value) {
    // Toggle editor's editable state
    editor.value.setEditable(!showPreview.value)
  }
}

// Mark document as complete
const markComplete = async () => {
  try {
    const response = await invoke<{ data: any }>('complete_document', {
      documentId: props.document.id
    })
    
    if (response.data) {
      emit('updated', response.data)
    }
  } catch (e) {
    console.error('Failed to mark document complete:', e)
  }
}

// Watch for document changes (skip initial load since onMounted handles it)
watch(() => props.document, () => {
  if (editor.value) {
    loadDocument()
  }
})

// Load content when component mounts
onMounted(() => {
  // Load document content if available
  if (props.document) {
    loadDocument()
  }
})

// Cleanup
onBeforeUnmount(() => {
  editor.value?.destroy()
})
</script>

<style scoped>
.document-editor {
  display: flex;
  flex-direction: column;
  height: 100%;
  min-height: 600px; /* Ensure minimum height */
  background-color: var(--color-background);
}

/* Editor Header */
.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-lg);
  background-color: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
}

.header-left {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.header-left h2 {
  margin: 0;
  font-size: 1.25rem;
  color: var(--color-text);
}

.header-right {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.save-status {
  font-size: 0.875rem;
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--radius-sm);
}

.save-status.saving {
  color: var(--color-primary-600);
  background-color: var(--color-primary-50);
}

.save-status.saved {
  color: var(--color-success);
  background-color: var(--color-success-50);
}

.save-status.error {
  color: var(--color-error);
  background-color: var(--color-error-50);
}

/* Editor Content */
.editor-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.editor-wrapper,
.preview-wrapper {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

/* Toolbar */
.editor-toolbar {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-sm) var(--spacing-lg);
  background-color: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
  flex-wrap: wrap;
}

.toolbar-btn {
  padding: var(--spacing-xs) var(--spacing-sm);
  background: none;
  border: 1px solid transparent;
  border-radius: var(--radius-sm);
  color: var(--color-text);
  cursor: pointer;
  font-size: 0.875rem;
  transition: all var(--transition-base);
}

.toolbar-btn:hover {
  background-color: var(--color-surface-variant);
  border-color: var(--color-border);
}

.toolbar-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.toolbar-btn.is-active {
  background-color: var(--color-primary-100);
  color: var(--color-primary-700);
  border-color: var(--color-primary-300);
}

.toolbar-divider {
  width: 1px;
  height: 24px;
  background-color: var(--color-border);
  margin: 0 var(--spacing-xs);
}

/* Editor Area */
.editor-area {
  flex: 1;
  overflow-y: auto;
  padding: var(--spacing-xl);
  background-color: var(--color-surface);
}

.editor-area :deep(.ProseMirror) {
  min-height: 100%;
  outline: none;
  font-size: 1rem;
  line-height: 1.6;
  color: var(--color-text);
}

.editor-area :deep(.ProseMirror p.is-editor-empty:first-child::before) {
  content: attr(data-placeholder);
  float: left;
  color: var(--color-text-secondary);
  pointer-events: none;
  height: 0;
}

/* Typography in editor */
.editor-area :deep(.ProseMirror h1) {
  font-size: 2rem;
  font-weight: 700;
  margin: 1.5rem 0 1rem;
}

.editor-area :deep(.ProseMirror h2) {
  font-size: 1.5rem;
  font-weight: 600;
  margin: 1.25rem 0 0.75rem;
}

.editor-area :deep(.ProseMirror h3) {
  font-size: 1.25rem;
  font-weight: 600;
  margin: 1rem 0 0.5rem;
}

.editor-area :deep(.ProseMirror p) {
  margin: 0.75rem 0;
}

.editor-area :deep(.ProseMirror ul),
.editor-area :deep(.ProseMirror ol) {
  padding-left: 1.5rem;
  margin: 0.75rem 0;
}

.editor-area :deep(.ProseMirror li) {
  margin: 0.25rem 0;
}

.editor-area :deep(.ProseMirror blockquote) {
  border-left: 3px solid var(--color-primary-300);
  padding-left: 1rem;
  margin: 1rem 0;
  color: var(--color-text-secondary);
}

.editor-area :deep(.ProseMirror hr) {
  border: none;
  border-top: 2px solid var(--color-border);
  margin: 2rem 0;
}

/* Preview mode styling */
.editor-area.preview-mode {
  background-color: var(--color-background);
  cursor: default;
}

.editor-area.preview-mode :deep(.ProseMirror) {
  padding: var(--spacing-2xl);
  max-width: 800px;
  margin: 0 auto;
}

/* Make sure content is not selectable in preview mode */
.editor-area.preview-mode :deep(.ProseMirror) {
  user-select: text;
}

/* Buttons */
.btn-icon {
  background: none;
  border: none;
  color: var(--color-text);
  cursor: pointer;
  font-size: 1rem;
  padding: var(--spacing-xs);
  transition: color var(--transition-base);
}

.btn-icon:hover {
  color: var(--color-primary-600);
}

.btn-primary {
  background-color: var(--color-primary-500);
  color: white;
  border: none;
  border-radius: var(--radius-md);
  padding: var(--spacing-sm) var(--spacing-lg);
  cursor: pointer;
  font-weight: 500;
  transition: background-color var(--transition-base);
}

.btn-primary:hover:not(:disabled) {
  background-color: var(--color-primary-600);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-secondary {
  background-color: var(--color-surface);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  padding: var(--spacing-sm) var(--spacing-lg);
  cursor: pointer;
  font-weight: 500;
  transition: all var(--transition-base);
}

.btn-secondary:hover {
  background-color: var(--color-surface-variant);
  border-color: var(--color-primary-300);
}

.btn-secondary.active {
  background-color: var(--color-primary-100);
  color: var(--color-primary-700);
  border-color: var(--color-primary-500);
}
</style>