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

      <!-- Main Content Area with Notes Panel -->
      <div class="main-wrapper">
        <main class="play-main" :class="{ 'notes-expanded': !notesCollapsed }">
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

        <!-- Collapsible Notes Panel -->
        <aside class="notes-panel" :class="{ collapsed: notesCollapsed }">
          <button class="notes-toggle" @click="toggleNotes">
            <span class="notes-toggle-icon">{{ notesCollapsed ? '&#9650;' : '&#9660;' }}</span>
            <span class="notes-toggle-label">Session Notes</span>
            <span v-if="notesSaving" class="notes-saving">Saving...</span>
            <span v-else-if="notesLastSaved" class="notes-saved">Saved</span>
          </button>

          <div class="notes-content" v-show="!notesCollapsed">
            <textarea
              v-model="notesContent"
              class="notes-textarea"
              placeholder="Type your session notes here... (auto-saves)"
              @input="handleNotesInput"
            ></textarea>
          </div>
        </aside>
      </div>
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
import type { Module, Document, Campaign } from '@/types'

const route = useRoute()
const router = useRouter()

const moduleId = computed(() => parseInt(route.params.id as string))
const module = ref<Module | null>(null)
const campaign = ref<Campaign | null>(null)
const documents = ref<Document[]>([])
const selectedDocument = ref<Document | null>(null)
const sidebarCollapsed = ref(false)
const documentsLoading = ref(true)
const documentLoading = ref(false)

// Notes state
const notesCollapsed = ref(true)
const notesContent = ref('')
const notesFilePath = ref('')
const notesSaving = ref(false)
const notesLastSaved = ref(false)
let saveTimeout: ReturnType<typeof setTimeout> | null = null

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

// Load module and campaign data
async function loadModule() {
  try {
    const response = await invoke<{ data: Module }>('get_module', {
      id: moduleId.value
    })
    module.value = response.data

    // Load campaign to get directory path
    if (module.value?.campaign_id) {
      const campaignResponse = await invoke<{ data: Campaign }>('get_campaign', {
        id: module.value.campaign_id
      })
      campaign.value = campaignResponse.data

      // Build notes file path
      if (campaign.value?.directory_path && module.value) {
        const moduleNumber = (module.value as any).module_number || 1
        const paddedNumber = String(moduleNumber).padStart(2, '0')
        notesFilePath.value = `${campaign.value.directory_path}/modules/module_${paddedNumber}/play-notes.md`

        // Load existing notes
        await loadNotes()
      }
    }
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

// Strip YAML frontmatter from markdown content
function stripFrontmatter(content: string): string {
  const frontmatterRegex = /^---\r?\n[\s\S]*?\r?\n---\r?\n?/
  return content.replace(frontmatterRegex, '')
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
      const content = stripFrontmatter(response.data)
      editor.value.commands.setContent(content)
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

// Notes panel
function toggleNotes() {
  notesCollapsed.value = !notesCollapsed.value
}

// Load notes from file
async function loadNotes() {
  if (!notesFilePath.value) return

  try {
    const response = await invoke<{ data: string }>('read_document_file', {
      filePath: notesFilePath.value
    })
    if (response.data) {
      notesContent.value = response.data
    }
  } catch (error) {
    // File might not exist yet, that's OK
    console.log('Notes file not found, will create on first save')
    notesContent.value = ''
  }
}

// Save notes to file
async function saveNotes() {
  if (!notesFilePath.value) return

  notesSaving.value = true
  notesLastSaved.value = false

  try {
    await invoke('save_document_file', {
      filePath: notesFilePath.value,
      content: notesContent.value
    })
    notesLastSaved.value = true
    // Clear the "Saved" indicator after 2 seconds
    setTimeout(() => {
      notesLastSaved.value = false
    }, 2000)
  } catch (error) {
    console.error('Failed to save notes:', error)
  } finally {
    notesSaving.value = false
  }
}

// Handle notes input with debounced auto-save
function handleNotesInput() {
  // Clear any pending save
  if (saveTimeout) {
    clearTimeout(saveTimeout)
  }

  // Schedule save after 1 second of inactivity
  saveTimeout = setTimeout(() => {
    saveNotes()
  }, 1000)
}

// Cleanup
onBeforeUnmount(() => {
  editor.value?.destroy()
  // Save any pending notes before unmount
  if (saveTimeout) {
    clearTimeout(saveTimeout)
    saveNotes()
  }
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

.main-wrapper {
  flex: 1;
  display: flex;
  flex-direction: column;
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
  overflow: hidden;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  min-height: 0;
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
  min-height: 0;
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
  min-height: 0;
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

/* Notes Panel */
.notes-panel {
  background: var(--color-surface);
  border-top: 1px solid var(--color-border);
  display: flex;
  flex-direction: column;
  transition: height 0.3s ease;
  height: 250px;
  min-height: 40px;
}

.notes-panel.collapsed {
  height: 40px;
}

.notes-toggle {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  background: var(--color-base-300);
  border: none;
  border-bottom: 1px solid var(--color-border);
  cursor: pointer;
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--color-text);
  text-align: left;
  width: 100%;
}

.notes-toggle:hover {
  background: var(--color-base-200);
}

.notes-toggle-icon {
  font-size: 0.75rem;
  color: var(--color-text-muted);
}

.notes-toggle-label {
  flex: 1;
}

.notes-saving {
  font-size: 0.75rem;
  color: var(--color-warning, #f59e0b);
  font-style: italic;
}

.notes-saved {
  font-size: 0.75rem;
  color: var(--color-success, #10b981);
}

.notes-content {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.notes-textarea {
  flex: 1;
  padding: 1rem;
  border: none;
  resize: none;
  font-family: inherit;
  font-size: 0.9rem;
  line-height: 1.6;
  background: var(--color-surface);
  color: var(--color-text);
}

.notes-textarea:focus {
  outline: none;
}

.notes-textarea::placeholder {
  color: var(--color-text-muted);
  font-style: italic;
}

/* Adjust main area when notes expanded */
.play-main.notes-expanded {
  flex: 1;
  min-height: 0;
}
</style>
