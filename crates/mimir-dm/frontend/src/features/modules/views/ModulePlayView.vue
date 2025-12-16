<template>
  <div class="play-mode-layout">
    <!-- Play Mode Header -->
    <header class="play-header">
      <div class="header-left">
        <button class="back-button" @click="navigateBack">
          <span class="back-icon">&larr;</span>
          Back to Module
        </button>
      </div>

      <div class="header-center">
        <h1 class="module-name">{{ module?.name || 'Loading...' }}</h1>
        <div class="play-mode-badge">PLAY MODE</div>
      </div>

      <div class="header-right">
        <button class="end-session-button" @click="handleEndSession">
          End Session
        </button>
      </div>
    </header>

    <!-- Main Play Area -->
    <div class="play-content">
      <!-- Collapsible Sidebar -->
      <aside class="play-sidebar" :class="{ collapsed: sidebarCollapsed }">
        <button class="sidebar-toggle" @click="toggleSidebar">
          {{ sidebarCollapsed ? '&raquo;' : '&laquo;' }}
        </button>

        <div class="sidebar-content" v-show="!sidebarCollapsed">
          <div class="sidebar-section">
            <h3>Quick Access</h3>
            <p class="placeholder-text">Monster cards and NPC info will appear here</p>
          </div>

          <div class="sidebar-section">
            <h3>Encounters</h3>
            <p class="placeholder-text">Encounter groups will be listed here</p>
          </div>
        </div>
      </aside>

      <!-- Main Content Area -->
      <main class="play-main">
        <!-- Document Tabs -->
        <div class="document-tabs" v-if="documents.length > 0">
          <button
            v-for="doc in documents"
            :key="doc.id"
            class="doc-tab"
            :class="{ active: selectedDocument?.id === doc.id }"
            @click="selectDocument(doc)"
          >
            {{ doc.title }}
          </button>
        </div>

        <!-- Document Viewer -->
        <div class="content-panel document-panel" v-if="selectedDocument">
          <div class="document-header">
            <h2>{{ selectedDocument.title }}</h2>
          </div>
          <div class="document-content">
            <div v-if="documentLoading" class="loading-state">
              Loading document...
            </div>
            <div v-else-if="editor" class="prose-content">
              <EditorContent :editor="editor" />
            </div>
          </div>
        </div>

        <!-- Fallback when no documents -->
        <div class="content-panel" v-else-if="!documentsLoading && documents.length === 0">
          <h2>Module Narrative</h2>
          <p class="placeholder-text">
            No module documents found. Create documents in the module prep view.
          </p>
        </div>

        <!-- Loading state -->
        <div class="content-panel" v-else-if="documentsLoading">
          <p class="placeholder-text">Loading documents...</p>
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import { Markdown } from 'tiptap-markdown-3'
import type { Module, Document } from '@/types'

const route = useRoute()
const router = useRouter()

const moduleId = computed(() => parseInt(route.params.id as string))
const module = ref<Module | null>(null)
const documents = ref<Document[]>([])
const selectedDocument = ref<Document | null>(null)
const sidebarCollapsed = ref(false)
const documentsLoading = ref(true)
const documentLoading = ref(false)

// TipTap editor for read-only document viewing
const editor = useEditor({
  content: '',
  editable: false,
  extensions: [
    StarterKit.configure({
      heading: {
        levels: [1, 2, 3, 4, 5, 6]
      }
    }),
    Markdown.configure({
      html: true,
      tightLists: true,
      tightListClass: 'tight',
      bulletListMarker: '-',
      linkify: false,
      breaks: false
    })
  ]
})

// Load module data
async function loadModule() {
  try {
    const response = await invoke<{ data: Module }>('get_module', {
      id: moduleId.value
    })
    module.value = response.data
  } catch (error) {
    console.error('Failed to load module:', error)
  }
}

// Load module documents
async function loadDocuments() {
  documentsLoading.value = true
  try {
    const response = await invoke<{ data: Document[] }>('get_module_documents', {
      request: {
        module_id: moduleId.value
      }
    })
    documents.value = response.data || []

    // Auto-select the first document (usually module overview)
    if (documents.value.length > 0) {
      // Try to find module_overview first, otherwise use first document
      const overview = documents.value.find(d => d.template_id === 'module_overview')
      selectDocument(overview || documents.value[0])
    }
  } catch (error) {
    console.error('Failed to load documents:', error)
  } finally {
    documentsLoading.value = false
  }
}

// Select and load a document
async function selectDocument(doc: Document) {
  selectedDocument.value = doc
  await loadDocumentContent(doc)
}

// Load document content
async function loadDocumentContent(doc: Document) {
  if (!doc.file_path) return

  documentLoading.value = true
  try {
    const response = await invoke<{ data: string }>('read_document_file', {
      filePath: doc.file_path
    })

    if (response.data && editor.value) {
      editor.value.commands.setContent(response.data)
    }
  } catch (error) {
    console.error('Failed to load document content:', error)
    if (editor.value) {
      editor.value.commands.setContent('*Failed to load document content*')
    }
  } finally {
    documentLoading.value = false
  }
}

// Navigation
function navigateBack() {
  router.push({ name: 'module-board', params: { id: moduleId.value } })
}

function handleEndSession() {
  if (confirm('End this play session and return to module prep?')) {
    navigateBack()
  }
}

// Sidebar
function toggleSidebar() {
  sidebarCollapsed.value = !sidebarCollapsed.value
}

// Cleanup
onBeforeUnmount(() => {
  editor.value?.destroy()
})

onMounted(async () => {
  await loadModule()
  await loadDocuments()
})
</script>

<style scoped>
.play-mode-layout {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  background: var(--color-base-200);
}

/* Header Styles */
.play-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.75rem 1.5rem;
  background: var(--color-base-300);
  border-bottom: 2px solid var(--color-accent, #e67e22);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.header-left,
.header-right {
  flex: 1;
}

.header-right {
  display: flex;
  justify-content: flex-end;
}

.header-center {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.25rem;
}

.module-name {
  font-size: 1.25rem;
  font-weight: 600;
  margin: 0;
  color: var(--color-text);
}

.play-mode-badge {
  font-size: 0.7rem;
  font-weight: 700;
  letter-spacing: 0.1em;
  padding: 0.2rem 0.6rem;
  background: var(--color-accent, #e67e22);
  color: white;
  border-radius: 0.25rem;
}

.back-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  background: transparent;
  border: 1px solid var(--color-border);
  border-radius: 0.375rem;
  color: var(--color-text);
  cursor: pointer;
  transition: all 0.2s;
}

.back-button:hover {
  background: var(--color-surface);
  border-color: var(--color-primary);
}

.back-icon {
  font-size: 1.1rem;
}

.end-session-button {
  padding: 0.5rem 1rem;
  background: var(--color-error, #dc3545);
  color: white;
  border: none;
  border-radius: 0.375rem;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s;
}

.end-session-button:hover {
  background: var(--color-error-dark, #c82333);
}

/* Content Area */
.play-content {
  flex: 1;
  display: flex;
  overflow: hidden;
}

/* Sidebar Styles */
.play-sidebar {
  width: 280px;
  background: var(--color-surface);
  border-right: 1px solid var(--color-border);
  display: flex;
  flex-direction: column;
  transition: width 0.3s ease;
  position: relative;
}

.play-sidebar.collapsed {
  width: 40px;
}

.sidebar-toggle {
  position: absolute;
  right: -12px;
  top: 1rem;
  width: 24px;
  height: 24px;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: 50%;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10;
  font-size: 0.8rem;
  color: var(--color-text-muted);
}

.sidebar-toggle:hover {
  background: var(--color-base-200);
  border-color: var(--color-primary);
}

.sidebar-content {
  padding: 1rem;
  overflow-y: auto;
  flex: 1;
}

.sidebar-section {
  margin-bottom: 1.5rem;
}

.sidebar-section h3 {
  font-size: 0.875rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--color-text-muted);
  margin-bottom: 0.75rem;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid var(--color-border);
}

/* Main Content */
.play-main {
  flex: 1;
  padding: 1.5rem;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

/* Document Tabs */
.document-tabs {
  display: flex;
  gap: 0.25rem;
  flex-wrap: wrap;
}

.doc-tab {
  padding: 0.5rem 1rem;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-bottom: none;
  border-radius: 0.375rem 0.375rem 0 0;
  cursor: pointer;
  font-size: 0.875rem;
  color: var(--color-text-muted);
  transition: all 0.2s;
}

.doc-tab:hover {
  background: var(--color-base-200);
  color: var(--color-text);
}

.doc-tab.active {
  background: var(--color-surface);
  color: var(--color-text);
  border-color: var(--color-accent, #e67e22);
  border-bottom: 2px solid var(--color-surface);
  margin-bottom: -1px;
  font-weight: 500;
}

/* Content Panel */
.content-panel {
  background: var(--color-surface);
  border-radius: 0.5rem;
  padding: 1.5rem;
  border: 1px solid var(--color-border);
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.content-panel h2 {
  font-size: 1.1rem;
  font-weight: 600;
  margin: 0 0 1rem 0;
  color: var(--color-text);
}

.document-panel {
  border-top-left-radius: 0;
}

.document-header {
  border-bottom: 1px solid var(--color-border);
  padding-bottom: 0.75rem;
  margin-bottom: 1rem;
}

.document-content {
  flex: 1;
  overflow-y: auto;
}

.loading-state {
  color: var(--color-text-muted);
  font-style: italic;
}

.placeholder-text {
  color: var(--color-text-muted);
  font-style: italic;
}

/* Prose styling for document content */
.prose-content {
  line-height: 1.7;
}

.prose-content :deep(h1) {
  font-size: 1.5rem;
  font-weight: 700;
  margin: 1.5rem 0 1rem 0;
  color: var(--color-text);
}

.prose-content :deep(h2) {
  font-size: 1.25rem;
  font-weight: 600;
  margin: 1.25rem 0 0.75rem 0;
  color: var(--color-text);
}

.prose-content :deep(h3) {
  font-size: 1.1rem;
  font-weight: 600;
  margin: 1rem 0 0.5rem 0;
  color: var(--color-text);
}

.prose-content :deep(p) {
  margin: 0.75rem 0;
}

.prose-content :deep(ul),
.prose-content :deep(ol) {
  margin: 0.75rem 0;
  padding-left: 1.5rem;
}

.prose-content :deep(li) {
  margin: 0.25rem 0;
}

.prose-content :deep(blockquote) {
  border-left: 3px solid var(--color-accent, #e67e22);
  margin: 1rem 0;
  padding-left: 1rem;
  color: var(--color-text-muted);
  font-style: italic;
}

.prose-content :deep(hr) {
  border: none;
  border-top: 1px solid var(--color-border);
  margin: 1.5rem 0;
}

.prose-content :deep(strong) {
  font-weight: 600;
}

.prose-content :deep(em) {
  font-style: italic;
}
</style>
