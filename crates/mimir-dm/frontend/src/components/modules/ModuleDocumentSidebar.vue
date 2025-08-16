<template>
  <div class="document-sidebar">
    <div class="sidebar-header">
      <h3 v-if="['active', 'completed'].includes(moduleStage)">Module Documents</h3>
      <h3 v-else>📋 Module Documents</h3>
    </div>
    
    <!-- Back to Campaign Button -->
    <div class="back-button-container" v-if="campaignId">
      <router-link :to="`/campaigns/${campaignId}/board`" class="back-to-campaign">
        ← Back to Campaign
      </router-link>
    </div>

    <!-- Document List Grouped by Stage -->
    <div class="document-content">
      <div v-if="loading" class="loading-state">
        Loading documents...
      </div>
      
      <div v-else-if="['active', 'completed'].includes(moduleStage)" class="active-documents">
        <!-- Simple list for active/completed modules -->
        <div class="document-items">
          <div 
            v-for="doc in getAllDocumentsForActive()" 
            :key="doc.templateId"
            class="document-item"
            :class="{ 
              selected: selectedDocument?.template_id === doc.templateId
            }"
          >
            <!-- Edit icon on the left -->
            <img 
              :src="getEditIcon()" 
              alt="Edit"
              class="document-icon"
              @click="handleDocumentClick(doc)"
              title="Edit document"
            />
            
            <!-- Document title -->
            <span 
              class="document-title"
              @click="handleDocumentClick(doc)"
            >
              {{ doc.title }}
            </span>
          </div>
        </div>
      </div>
      
      <div v-else class="stage-groups">
        <!-- Dynamic Stage Documents from Board Configuration -->
        <div 
          v-for="stage in boardConfig?.stages || []" 
          :key="stage.key"
          v-show="isStageAccessible(stage.key)"
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
  moduleId: number
  moduleStage: string
  boardConfig: any
  campaignId?: number
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

// Get documents for a specific stage
const getStageDocuments = (stage: string) => {
  const templates = stageDocuments.value[stage] || []
  const stageDocumentList = templates.map((template: any) => {
    const instance = documents.value.find(doc => 
      doc.template_id === template.templateId && doc.module_id === props.moduleId
    )
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

// Check if a stage is accessible based on module progress
const isStageAccessible = (stage: string) => {
  if (!props.boardConfig) return false
  const stageOrder = props.boardConfig.stages.map((s: any) => s.key)
  const currentIndex = stageOrder.indexOf(props.moduleStage)
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

// Get all documents for active/completed modules (simplified view)
const getAllDocumentsForActive = () => {
  const allDocs: any[] = []
  
  // Collect all documents from all stages (both required and optional)
  if (props.boardConfig) {
    for (const stage of props.boardConfig.stages) {
      const templates = stageDocuments.value[stage.key] || []
      for (const template of templates) {
        const instance = documents.value.find(doc => 
          doc.template_id === template.templateId && doc.module_id === props.moduleId
        )
        
        // Include all documents (with or without instance)
        allDocs.push({
          ...template,
          instance
        })
      }
    }
  }
  
  // Sort alphabetically by title
  return allDocs.sort((a, b) => a.title.localeCompare(b.title))
}

// Load all documents for the module
const loadDocuments = async () => {
  loading.value = true
  error.value = null

  try {
    const response = await invoke<{ data: Document[] }>('get_module_documents', {
      request: {
        module_id: props.moduleId
      }
    })
    documents.value = response.data || []
    console.log('Loaded documents for module', props.moduleId, ':', documents.value)
  } catch (e) {
    console.error('Failed to load documents:', e)
    error.value = 'Failed to load documents'
  } finally {
    loading.value = false
  }
}

// Handle document click
const handleDocumentClick = async (doc: any) => {
  console.log('Document clicked:', doc)
  const stage = getDocumentStage(doc.templateId)
  
  // Check if stage is accessible
  if (!isStageAccessible(stage)) {
    console.log('Stage is locked:', stage)
    return
  }
  
  // If document doesn't exist in database, create it first
  if (!doc.instance) {
    try {
      // First get the module to get the campaign ID
      const moduleResponse = await invoke<{ success: boolean; data: any }>('get_module', {
        id: props.moduleId
      })
      
      if (!moduleResponse.success || !moduleResponse.data) {
        console.error('Failed to get module info')
        return
      }
      
      const module = moduleResponse.data
      
      // Create the document in the database AND on disk
      const response = await invoke<{ data: Document }>('create_document_from_template', {
        campaignId: module.campaign_id,
        moduleId: props.moduleId,
        templateId: doc.templateId
      })
      
      if (response.data) {
        // Check if document already exists and update or add
        const existingIndex = documents.value.findIndex(d => 
          d.template_id === response.data.template_id && d.module_id === props.moduleId
        )
        
        if (existingIndex !== -1) {
          documents.value[existingIndex] = response.data
        } else {
          documents.value.push(response.data)
        }
        
        // Force reactivity update to make checkbox appear
        documents.value = [...documents.value]
        
        // Select the document
        selectDocument(response.data)
      }
    } catch (e) {
      console.error('Failed to create document from template:', e)
    }
  } else {
    console.log('Selecting existing document:', doc.instance)
    selectDocument(doc.instance)
  }
}

// Get which stage a template belongs to
const getDocumentStage = (templateId: string): string => {
  for (const [stage, docs] of Object.entries(stageDocuments.value)) {
    if ((docs as any[]).some(d => d.templateId === templateId)) {
      return stage
    }
  }
  return props.moduleStage
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

// Watch for module or stage changes
watch([() => props.moduleId, () => props.moduleStage], () => {
  loadDocuments()
})

onMounted(() => {
  loadDocuments()
})

// Expose loadDocuments for parent to call
defineExpose({
  loadDocuments
})
</script>

<style scoped>
/* Module document sidebar - inherits common styles from components.css */
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

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-xl);
  text-align: center;
  color: var(--color-text-secondary);
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

.document-icon.locked {
  cursor: not-allowed;
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

/* Active Documents View (simplified for active/completed states) */
.active-documents {
  padding-top: var(--spacing-sm);
}

.active-documents .document-items {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.active-documents .document-item {
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-sm);
  transition: background-color var(--transition-fast);
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.active-documents .document-item:hover {
  background-color: var(--color-surface-variant);
}

.active-documents .document-title {
  cursor: pointer;
}

/* Back to Campaign Button */
.back-button-container {
  padding: var(--spacing-md);
  border-bottom: 1px solid var(--color-border);
}

.back-to-campaign {
  display: inline-flex;
  align-items: center;
  padding: var(--spacing-sm) var(--spacing-md);
  background-color: var(--color-surface-variant);
  color: var(--color-text);
  text-decoration: none;
  border-radius: var(--radius-sm);
  font-size: 0.875rem;
  font-weight: 500;
  transition: all var(--transition-base);
  width: 100%;
  justify-content: center;
}

.back-to-campaign:hover {
  background-color: var(--color-primary-100);
  color: var(--color-primary-600);
  transform: translateX(-2px);
}
</style>