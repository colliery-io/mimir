<template>
  <div class="document-sidebar">
    <div class="sidebar-header">
      <h3>📋 Documents</h3>
    </div>

    <!-- Document List Grouped by Stage -->
    <div class="document-content">
      <div v-if="loading" class="loading-state">
        Loading documents...
      </div>
      
      <div v-else class="stage-groups">
        <!-- Dynamic Stage Documents from Board Configuration -->
        <div 
          v-for="stage in boardConfig?.stages || []" 
          :key="stage.key"
          v-show="isStageAccessible(stage.key) || stage.key === 'concept'"
          class="stage-group"
        >
          <div class="stage-header">
            <h4>{{ stage.display_name }} ({{ getStageDocuments(stage.key).completed }}/{{ getStageDocuments(stage.key).total }})</h4>
            <div class="progress-bar">
              <div 
                class="progress-fill" 
                :style="{ width: getStageDocuments(stage.key).percentage + '%' }"
              ></div>
            </div>
          </div>
          <div class="document-items">
            <div 
              v-for="doc in getStageDocuments(stage.key).documents" 
              :key="doc.templateId"
              class="document-item"
              :class="{ 
                selected: selectedDocument?.template_id === doc.templateId,
                completed: doc.instance?.completed_at,
                locked: !isStageAccessible(stage.key)
              }"
            >
              <!-- Edit icon on the left (always visible, clickable) -->
              <img 
                v-if="!isStageAccessible(stage.key)"
                :src="getLockedIcon()" 
                alt="Locked"
                class="document-icon locked"
                title="Stage not yet accessible"
              />
              <img 
                v-else
                :src="getEditIcon()" 
                alt="Edit"
                class="document-icon"
                @click="handleDocumentClick(doc)"
                title="Edit document"
              />
              
              <!-- Document title (also clickable) -->
              <span 
                class="document-title" 
                :class="{ optional: !doc.required }"
                @click="handleDocumentClick(doc)"
              >
                {{ doc.title }}
                <span v-if="!doc.required" class="optional-label">(Optional)</span>
              </span>
              
              <!-- Checkmark on the right for completion -->
              <button 
                v-if="doc.instance && isStageAccessible(stage.key)"
                class="completion-checkbox"
                :class="{ checked: doc.instance?.completed_at }"
                @click.stop="toggleDocumentCompletion(doc)"
                :title="doc.instance?.completed_at ? 'Mark as incomplete' : 'Mark as complete'"
              >
                <span v-if="doc.instance?.completed_at">✓</span>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useThemeStore } from '../../stores/theme'
import { debugDocument } from '../../utils/debug'

// Import icon images
import lightEditIcon from '../../assets/images/light-edit.png'
import lightLockedIcon from '../../assets/images/light-locked.png'
import darkEditIcon from '../../assets/images/dark-edit.png'
import darkLockedIcon from '../../assets/images/dark-locked.png'
import hyperEditIcon from '../../assets/images/hyper-edit.png'
import hyperLockedIcon from '../../assets/images/hyper-locked.png'

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
  campaignStage: string
  boardConfig: any
}>()

const emit = defineEmits<{
  selectDocument: [document: Document]
  createDocument: []
  documentCompletionChanged: [document: Document]
}>()

// Get document templates from board configuration
const stageDocuments = computed(() => {
  if (!props.boardConfig) return {}
  
  const documents: Record<string, any[]> = {}
  
  for (const stage of props.boardConfig.stages) {
    documents[stage.key] = [
      ...stage.required_documents.map((docId: string) => ({
        templateId: docId,
        title: docId.replace(/[-_]/g, ' ').split(' ').map((word: string) => 
          word.charAt(0).toUpperCase() + word.slice(1)
        ).join(' '),
        required: true
      })),
      ...stage.optional_documents.map((docId: string) => ({
        templateId: docId,
        title: docId.replace(/[-_]/g, ' ').split(' ').map((word: string) => 
          word.charAt(0).toUpperCase() + word.slice(1)
        ).join(' '),
        required: false
      }))
    ]
  }
  
  return documents
})

// State
const documents = ref<Document[]>([])
const selectedDocument = ref<Document | null>(null)
const loading = ref(false)
const error = ref<string | null>(null)

// Theme store for icon selection
const themeStore = useThemeStore()

// Icon mapping
const iconMap = {
  light: {
    edit: lightEditIcon,
    locked: lightLockedIcon
  },
  dark: {
    edit: darkEditIcon,
    locked: darkLockedIcon
  },
  hyper: {
    edit: hyperEditIcon,
    locked: hyperLockedIcon
  }
}

// Stage documents are now computed dynamically from board configuration

// Get documents for a specific stage
const getStageDocuments = (stage: string) => {
  const templates = stageDocuments.value[stage] || []
  const stageDocumentList = templates.map((template: any) => {
    // Simple matching - everything uses snake_case now
    const instance = documents.value.find(doc => 
      doc.template_id === template.templateId
    )
    console.log('Document mapping:', {
      templateId: template.templateId,
      instance: instance,
      hasInstance: !!instance,
      documents: documents.value
    })
    return {
      ...template,
      instance
    }
  })
  
  // Only count required documents for completion tracking
  const requiredDocs = stageDocumentList.filter((doc: any) => doc.required)
  const completed = requiredDocs.filter((doc: any) => doc.instance?.completed_at).length
  const total = requiredDocs.length
  const percentage = total > 0 ? Math.round((completed / total) * 100) : 0
  
  return {
    documents: stageDocumentList,
    completed,
    total,
    percentage
  }
}

// Check if a stage is accessible based on campaign progress
const isStageAccessible = (stage: string) => {
  if (!props.boardConfig) return false
  const stageOrder = props.boardConfig.stages.map((s: any) => s.key)
  const currentIndex = stageOrder.indexOf(props.campaignStage)
  const checkIndex = stageOrder.indexOf(stage)
  return checkIndex <= currentIndex
}

// Get edit icon for current theme
const getEditIcon = (): string => {
  const theme = themeStore.currentTheme as 'light' | 'dark' | 'hyper'
  return iconMap[theme]?.edit || lightEditIcon
}

// Get locked icon for current theme
const getLockedIcon = (): string => {
  const theme = themeStore.currentTheme as 'light' | 'dark' | 'hyper'
  return iconMap[theme]?.locked || lightLockedIcon
}

// Get which stage a template belongs to
const getDocumentStage = (templateId: string): string => {
  for (const [stage, docs] of Object.entries(stageDocuments.value)) {
    if ((docs as any[]).some(d => d.templateId === templateId)) {
      return stage
    }
  }
  return 'concept'
}

// Load all documents for the campaign
const loadDocuments = async () => {
  loading.value = true
  error.value = null

  try {
    const response = await invoke<{ data: Document[] }>('get_campaign_documents', {
      campaignId: props.campaignId
    })
    documents.value = response.data || []
  } catch (e) {
    console.error('Failed to load documents:', e)
    error.value = 'Failed to load documents'
  } finally {
    loading.value = false
  }
}

// Handle document click
const handleDocumentClick = async (doc: any) => {
  debugDocument('click', { doc, stage: getDocumentStage(doc.templateId) })
  console.log('Document clicked:', doc)
  const stage = getDocumentStage(doc.templateId)
  
  // Check if stage is accessible
  if (!isStageAccessible(stage)) {
    debugDocument('stage-locked', { stage })
    console.log('Stage is locked:', stage)
    return
  }
  
  // If document doesn't exist in database, just create a simple object pointing to the file
  if (!doc.instance) {
    // The file already exists on disk, just point to it
    const filePath = `${props.boardConfig.stages[0].key === 'concept' ? 
      props.campaignId : props.campaignId}/${doc.templateId.replace(/_/g, '-')}.md`
    
    // Get campaign info to build the full path
    const campaignResponse = await invoke<{ success: boolean; data: any }>('get_campaign', {
      id: props.campaignId
    })
    
    if (campaignResponse.success && campaignResponse.data) {
      const simpleDoc = {
        id: -1, // Use -1 as temporary ID to indicate it's not in database
        campaign_id: props.campaignId,
        template_id: doc.templateId,
        document_type: doc.templateId.replace(/-/g, '_'),
        title: doc.title,
        file_path: `${campaignResponse.data.directory_path}/${doc.templateId.replace(/_/g, '-')}.md`,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
        completed_at: null,
        module_id: null,
        session_id: null
      } as Document
      
      console.log('Opening existing file:', simpleDoc.file_path)
      
      // Add to documents array so it shows as existing
      const existingIndex = documents.value.findIndex(d => d.template_id === doc.templateId)
      if (existingIndex === -1) {
        documents.value.push(simpleDoc)
      } else {
        documents.value[existingIndex] = simpleDoc
      }
      
      // Select the document
      selectDocument(simpleDoc)
    }
  } else {
    debugDocument('selecting-existing', { instance: doc.instance })
    console.log('Selecting existing document:', doc.instance)
    selectDocument(doc.instance)
  }
}

// Create a new document from template
const createDocument = async (templateId: string, title: string) => {
  try {
    console.log('Creating document with params:', {
      campaignId: props.campaignId,
      templateId: templateId,
      propsType: typeof props.campaignId
    })
    
    // Use create_document_from_template which creates both file and DB record
    const response = await invoke<{ success: boolean; data: Document }>('create_document_from_template', {
      campaignId: props.campaignId,
      templateId: templateId
    })
    
    console.log('Document creation response:', response)
    
    if (response.success && response.data) {
      // Add the new document to our list
      documents.value.push(response.data)
      // Select it immediately
      selectDocument(response.data)
    } else {
      console.error('Document creation failed:', response)
    }
  } catch (e) {
    console.error('Failed to create document:', e)
  }
}

// Select a document
const selectDocument = (doc: Document) => {
  selectedDocument.value = doc
  emit('selectDocument', doc)
}

// Toggle document completion status
const toggleDocumentCompletion = async (doc: any) => {
  if (!doc.instance) return
  
  try {
    const newCompletedAt = doc.instance.completed_at ? null : new Date().toISOString()
    
    // All documents should be in the database now, so always update via backend
    const response = await invoke<{ success: boolean; data: Document }>('update_document', {
      documentId: doc.instance.id,
      update: {
        completed_at: newCompletedAt,
        updated_at: new Date().toISOString()
      }
    })
    
    if (response.success && response.data) {
      // Update the document in our local list
      const index = documents.value.findIndex(d => d.id === doc.instance.id)
      if (index !== -1) {
        documents.value[index] = response.data
      }
      
      // Also update the instance reference
      doc.instance = response.data
      
      // Force reactivity update
      documents.value = [...documents.value]
      
      // Emit completion status change
      emit('documentCompletionChanged', response.data)
    }
  } catch (e) {
    console.error('Failed to toggle document completion:', e)
  }
}

// Watch for campaign or stage changes
watch([() => props.campaignId, () => props.campaignStage], () => {
  loadDocuments()
})

onMounted(() => {
  loadDocuments()
})
</script>

<style scoped>
/* Campaign document sidebar uses common sidebar styles from components.css */
.document-sidebar {
  width: 320px;
  height: 100%;
  background-color: var(--color-surface);
  border-right: 1px solid var(--color-border);
  display: flex;
  flex-direction: column;
}

/* Stage Groups */
.stage-groups {
  display: flex;
  flex-direction: column;
}

.stage-group {
  padding: var(--spacing-sm) 0;
  border-bottom: 1px solid var(--color-border);
}

.stage-group:last-child {
  border-bottom: none;
}

.stage-header {
  padding: 0 var(--spacing-sm);
  margin-bottom: var(--spacing-sm);
}

.stage-header h4 {
  margin: 0 0 var(--spacing-xs) 0;
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--color-text-secondary);
  text-transform: uppercase;
}

/* Progress Bar */
.progress-bar {
  height: 16px;
  background-color: var(--color-surface-variant);
  border-radius: 8px;
  overflow: hidden;
  position: relative;
}

.progress-fill {
  height: 100%;
  background-color: var(--color-primary-400);
  border-radius: 8px;
  transition: width var(--transition-base);
  position: relative;
}

.progress-bar::after {
  content: '';
  position: absolute;
  inset: 0;
  background: repeating-linear-gradient(
    45deg,
    transparent,
    transparent 10px,
    rgba(0, 0, 0, 0.05) 10px,
    rgba(0, 0, 0, 0.05) 20px
  );
  border-radius: 8px;
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
  padding: var(--spacing-sm) var(--spacing-md);
  cursor: pointer;
  transition: all var(--transition-base);
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  border-radius: var(--radius-sm);
  border: 2px solid transparent;
}

.document-item:hover:not(.locked) {
  border-color: var(--color-border);
}

.document-item.selected {
  border-color: var(--color-primary-400);
}

.document-item.completed {
  opacity: 0.8;
}

.document-item.locked {
  opacity: 0.5;
  cursor: not-allowed;
}

.document-icon {
  width: 24px;
  height: 24px;
  flex-shrink: 0;
  object-fit: contain;
}

.document-icon-text {
  width: 24px;
  height: 24px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-success);
  font-weight: bold;
  font-size: 1.125rem;
}

.document-icon-placeholder {
  width: 24px;
  height: 24px;
  flex-shrink: 0;
}

.document-title {
  font-size: 0.875rem;
  color: var(--color-text);
  flex: 1;
}

/* Optional document styling */
.document-title.optional {
  font-style: italic;
}

.optional-label {
  font-size: 0.75rem;
  color: var(--color-text-secondary);
  font-style: normal;
  margin-left: var(--spacing-xs);
}

/* Completion Checkbox */
.completion-checkbox {
  width: 24px;
  height: 24px;
  flex-shrink: 0;
  border: 2px solid var(--color-border);
  border-radius: 4px;
  background: transparent;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--transition-base);
  margin-left: auto;
}

.completion-checkbox:hover {
  border-color: var(--color-primary-400);
  background-color: var(--color-primary-50);
}

.completion-checkbox.checked {
  background-color: var(--color-success);
  border-color: var(--color-success);
  color: white;
}

.completion-checkbox.checked:hover {
  background-color: var(--color-success-dark);
  border-color: var(--color-success-dark);
}

</style>