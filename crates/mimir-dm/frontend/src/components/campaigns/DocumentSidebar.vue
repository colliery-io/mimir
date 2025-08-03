<template>
  <div class="document-sidebar">
    <div class="sidebar-header">
      <h3>Campaign Documents</h3>
    </div>

    <!-- Tab Navigation -->
    <div class="tab-navigation">
      <button 
        class="tab-button" 
        :class="{ active: activeTab === 'active' }"
        @click="activeTab = 'active'"
      >
        <span class="tab-icon">📄</span>
        Active Documents
        <span class="tab-count" v-if="incompleteDocuments.length">{{ incompleteDocuments.length }}</span>
      </button>
      <button 
        class="tab-button" 
        :class="{ active: activeTab === 'completed' }"
        @click="activeTab = 'completed'"
      >
        <span class="tab-icon">✓</span>
        Completed
        <span class="tab-count" v-if="completedDocuments.length">{{ completedDocuments.length }}</span>
      </button>
    </div>

    <!-- Document Lists -->
    <div class="document-content">
      <!-- Active Documents -->
      <div v-if="activeTab === 'active'" class="document-list">
        <div v-if="loading" class="loading-state">
          Loading documents...
        </div>
        <div v-else-if="incompleteDocuments.length === 0" class="empty-state">
          <p>No active documents</p>
          <button class="btn-small btn-primary" @click="createNewDocument">
            Create Document
          </button>
        </div>
        <div v-else class="document-items">
          <div 
            v-for="doc in incompleteDocuments" 
            :key="doc.id"
            class="document-item"
            :class="{ selected: selectedDocument?.id === doc.id }"
            @click="selectDocument(doc)"
          >
            <div class="document-info">
              <h4>{{ doc.title }}</h4>
              <p class="document-type">{{ formatDocumentType(doc.document_type) }}</p>
              <p class="document-level">{{ getDocumentLevel(doc) }}</p>
            </div>
            <div class="document-actions">
              <button 
                class="btn-icon" 
                @click.stop="markAsComplete(doc)"
                title="Mark as complete"
              >
                ✓
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Completed Documents -->
      <div v-if="activeTab === 'completed'" class="document-list">
        <div v-if="loading" class="loading-state">
          Loading documents...
        </div>
        <div v-else-if="completedDocuments.length === 0" class="empty-state">
          <p>No completed documents</p>
        </div>
        <div v-else class="document-items">
          <div 
            v-for="doc in completedDocuments" 
            :key="doc.id"
            class="document-item completed"
            @click="selectDocument(doc)"
          >
            <div class="document-info">
              <h4>{{ doc.title }}</h4>
              <p class="document-type">{{ formatDocumentType(doc.document_type) }}</p>
              <p class="document-date">Completed: {{ formatDate(doc.completed_at) }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Quick Actions -->
    <div class="sidebar-footer">
      <button class="btn-block btn-secondary" @click="createNewDocument">
        + New Document
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface Document {
  id: number
  campaign_id: number
  module_id: number | null
  session_id: number | null
  template_id: string
  document_type: string
  title: string
  file_path: string
  created_at: string
  updated_at: string
  completed_at: string | null
}

const props = defineProps<{
  campaignId: number
}>()

const emit = defineEmits<{
  selectDocument: [document: Document]
  createDocument: []
}>()

// State
const activeTab = ref<'active' | 'completed'>('active')
const incompleteDocuments = ref<Document[]>([])
const completedDocuments = ref<Document[]>([])
const selectedDocument = ref<Document | null>(null)
const loading = ref(false)
const error = ref<string | null>(null)

// Load documents
const loadDocuments = async () => {
  loading.value = true
  error.value = null

  try {
    // Load incomplete documents
    const incompleteResponse = await invoke<{ data: Document[] }>('get_incomplete_documents', {
      campaignId: props.campaignId
    })
    incompleteDocuments.value = incompleteResponse.data || []

    // Load completed documents
    const completedResponse = await invoke<{ data: Document[] }>('get_completed_documents', {
      campaignId: props.campaignId
    })
    completedDocuments.value = completedResponse.data || []
  } catch (e) {
    console.error('Failed to load documents:', e)
    error.value = 'Failed to load documents'
  } finally {
    loading.value = false
  }
}

// Mark document as complete
const markAsComplete = async (doc: Document) => {
  try {
    const response = await invoke<{ data: Document }>('complete_document', {
      documentId: doc.id
    })
    
    if (response.data) {
      // Move from incomplete to completed
      incompleteDocuments.value = incompleteDocuments.value.filter(d => d.id !== doc.id)
      completedDocuments.value.unshift(response.data)
      
      // Clear selection if this was selected
      if (selectedDocument.value?.id === doc.id) {
        selectedDocument.value = null
      }
    }
  } catch (e) {
    console.error('Failed to complete document:', e)
  }
}

// Select a document
const selectDocument = (doc: Document) => {
  selectedDocument.value = doc
  emit('selectDocument', doc)
}

// Create new document
const createNewDocument = () => {
  emit('createDocument')
}

// Get document level display
const getDocumentLevel = (doc: Document): string => {
  if (doc.session_id) return 'Session'
  if (doc.module_id) return 'Module'
  if (doc.document_type === 'handout') return 'Handout'
  return 'Campaign'
}

// Format document type for display
const formatDocumentType = (type: string): string => {
  return type.split('_').map(word => 
    word.charAt(0).toUpperCase() + word.slice(1)
  ).join(' ')
}

// Format date
const formatDate = (dateStr: string | null): string => {
  if (!dateStr) return ''
  const date = new Date(dateStr)
  return date.toLocaleDateString()
}

// Watch for campaign changes
watch(() => props.campaignId, () => {
  loadDocuments()
})

onMounted(() => {
  loadDocuments()
})
</script>

<style scoped>
.document-sidebar {
  width: 320px;
  height: 100%;
  background-color: var(--color-surface);
  border-right: 1px solid var(--color-border);
  display: flex;
  flex-direction: column;
}

.sidebar-header {
  padding: var(--spacing-lg);
  border-bottom: 1px solid var(--color-border);
}

.sidebar-header h3 {
  margin: 0;
  font-size: 1.125rem;
  color: var(--color-text);
}

/* Tab Navigation */
.tab-navigation {
  display: flex;
  border-bottom: 1px solid var(--color-border);
}

.tab-button {
  flex: 1;
  padding: var(--spacing-md);
  background: none;
  border: none;
  border-bottom: 2px solid transparent;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-xs);
  color: var(--color-text-secondary);
  font-size: 0.875rem;
  transition: all var(--transition-base);
}

.tab-button:hover {
  background-color: var(--color-surface-variant);
}

.tab-button.active {
  color: var(--color-primary-500);
  border-bottom-color: var(--color-primary-500);
}

.tab-icon {
  font-size: 1rem;
}

.tab-count {
  background-color: var(--color-surface-variant);
  color: var(--color-text-secondary);
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 0.75rem;
  font-weight: 600;
}

.tab-button.active .tab-count {
  background-color: var(--color-primary-100);
  color: var(--color-primary-600);
}

/* Document Content */
.document-content {
  flex: 1;
  overflow-y: auto;
  padding: var(--spacing-md);
}

.document-list {
  height: 100%;
}

.loading-state,
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-xl);
  text-align: center;
  color: var(--color-text-secondary);
}

.empty-state p {
  margin-bottom: var(--spacing-md);
}

/* Document Items */
.document-items {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.document-item {
  background-color: var(--color-surface-variant);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  padding: var(--spacing-md);
  cursor: pointer;
  transition: all var(--transition-base);
  display: flex;
  justify-content: space-between;
  align-items: start;
}

.document-item:hover {
  border-color: var(--color-primary-300);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.document-item.selected {
  border-color: var(--color-primary-500);
  background-color: var(--color-primary-50);
}

.document-item.completed {
  opacity: 0.8;
}

.document-info h4 {
  margin: 0 0 var(--spacing-xs) 0;
  font-size: 0.875rem;
  color: var(--color-text);
}

.document-type,
.document-level,
.document-date {
  margin: 0;
  font-size: 0.75rem;
  color: var(--color-text-secondary);
}

.document-level {
  color: var(--color-primary-600);
  font-weight: 500;
}

.document-actions {
  display: flex;
  gap: var(--spacing-xs);
}

/* Buttons */
.btn-icon {
  width: 32px;
  height: 32px;
  padding: 0;
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--transition-base);
  color: var(--color-text-secondary);
}

.btn-icon:hover {
  background-color: var(--color-success-100);
  border-color: var(--color-success);
  color: var(--color-success);
}

.btn-small {
  padding: var(--spacing-xs) var(--spacing-md);
  font-size: 0.875rem;
  border-radius: var(--radius-sm);
  border: none;
  cursor: pointer;
  transition: all var(--transition-base);
}

.btn-primary {
  background-color: var(--color-primary-500);
  color: white;
}

.btn-primary:hover {
  background-color: var(--color-primary-600);
}

.btn-block {
  width: 100%;
  padding: var(--spacing-md);
  border: none;
  border-radius: var(--radius-md);
  cursor: pointer;
  font-weight: 500;
  transition: all var(--transition-base);
}

.btn-secondary {
  background-color: var(--color-surface-variant);
  color: var(--color-text);
  border: 1px solid var(--color-border);
}

.btn-secondary:hover {
  background-color: var(--color-surface);
  border-color: var(--color-primary-300);
}

/* Sidebar Footer */
.sidebar-footer {
  padding: var(--spacing-md);
  border-top: 1px solid var(--color-border);
}
</style>