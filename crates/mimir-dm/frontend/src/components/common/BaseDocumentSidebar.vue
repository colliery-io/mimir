<template>
  <div class="document-sidebar">
    <div class="sidebar-header">
      <h3 v-if="isActiveStage">{{ sidebarTitle }}</h3>
      <h3 v-else>📋 Documents</h3>
    </div>
    
    <!-- Optional back button slot -->
    <slot name="back-button" />

    <!-- Document List -->
    <div class="document-content">
      <div v-if="loading" class="loading-state">
        Loading documents...
      </div>
      
      <!-- Active/Completed Stage View -->
      <div v-else-if="isActiveStage" class="active-documents">
        <div class="document-items">
          <div 
            v-for="doc in getAllDocumentsForActive()" 
            :key="doc.templateId"
            class="document-item"
            :class="{ 
              selected: selectedDocument?.template_id === doc.templateId
            }"
          >
            <img 
              :src="getEditIcon" 
              alt="Edit"
              class="document-icon"
              @click="handleDocumentClick(doc)"
              title="Edit document"
            />
            
            <span 
              class="document-title"
              @click="handleDocumentClick(doc)"
            >
              {{ doc.title }}
            </span>
          </div>
        </div>
      </div>
      
      <!-- Stage-based View -->
      <div v-else class="stage-groups">
        <div 
          v-for="stage in boardConfig?.stages || []" 
          :key="stage.key"
          v-show="isStageAccessible(stage.key) || showAllStages"
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
              <img 
                v-if="!isStageAccessible(stage.key)"
                :src="getLockedIcon" 
                alt="Locked"
                class="document-icon locked"
                title="Stage not yet accessible"
              />
              <img 
                v-else
                :src="getEditIcon" 
                alt="Edit"
                class="document-icon"
                @click="handleDocumentClick(doc)"
                title="Edit document"
              />
              
              <span 
                class="document-title" 
                :class="{ optional: !doc.required }"
                @click="handleDocumentClick(doc)"
              >
                {{ doc.title }}
                <span v-if="!doc.required" class="optional-label">(Optional)</span>
              </span>
              
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
import { useThemeIcons } from '../../composables/useThemeIcons'
import { useApiCall } from '../../composables/useApiCall'
import { debugDocument } from '../../utils/debug'
import type { Document, BoardConfig, EntityType, StageDocuments } from '../../types'

interface Props {
  entityId: number
  entityType: EntityType
  entityStage: string
  boardConfig: BoardConfig | null
  activeStages?: string[]
  showAllStages?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  activeStages: () => ['active', 'concluding', 'completed'],
  showAllStages: false
})

const emit = defineEmits<{
  selectDocument: [document: Document]
  createDocument: []
  documentCompletionChanged: [document: Document]
}>()

// Use composables
const { getEditIcon, getLockedIcon } = useThemeIcons()
const { loading, error, execute: loadDocuments } = useApiCall<Document[]>()

// State
const documents = ref<Document[]>([])
const selectedDocument = ref<Document | null>(null)

// Computed properties
const sidebarTitle = computed(() => {
  const entityName = props.entityType.charAt(0).toUpperCase() + props.entityType.slice(1)
  return `${entityName} Documents`
})

const isActiveStage = computed(() => {
  return props.activeStages.includes(props.entityStage)
})

// Get document templates from board configuration
const stageDocuments = computed(() => {
  if (!props.boardConfig) return {}
  
  const documentsByStage: Record<string, any[]> = {}
  
  for (const stage of props.boardConfig.stages) {
    documentsByStage[stage.key] = [
      ...(stage as any).required_documents?.map((docId: string) => ({
        templateId: docId,
        title: formatDocumentTitle(docId),
        required: true
      })) || [],
      ...(stage as any).optional_documents?.map((docId: string) => ({
        templateId: docId,
        title: formatDocumentTitle(docId),
        required: false
      })) || []
    ]
  }
  
  return documentsByStage
})

// Helper functions
const formatDocumentTitle = (docId: string): string => {
  return docId
    .replace(/[-_]/g, ' ')
    .split(' ')
    .map((word: string) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ')
}

const getStageDocuments = (stage: string): StageDocuments => {
  const templates = stageDocuments.value[stage] || []
  const stageDocumentList = templates.map((template: any) => {
    const instance = documents.value.find(doc => 
      doc.template_id === template.templateId
    )
    return {
      ...template,
      instance
    }
  })
  
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

const isStageAccessible = (stage: string): boolean => {
  if (!props.boardConfig) return false
  const stageOrder = props.boardConfig.stages.map((s) => s.key)
  const currentIndex = stageOrder.indexOf(props.entityStage)
  const checkIndex = stageOrder.indexOf(stage)
  return checkIndex <= currentIndex
}

const getDocumentStage = (templateId: string): string => {
  for (const [stage, docs] of Object.entries(stageDocuments.value)) {
    if ((docs as any[]).some(d => d.templateId === templateId)) {
      return stage
    }
  }
  return props.boardConfig?.stages[0]?.key || ''
}

const getAllDocumentsForActive = () => {
  const allDocs: any[] = []
  
  if (props.boardConfig) {
    for (const stage of props.boardConfig.stages) {
      const templates = stageDocuments.value[stage.key] || []
      for (const template of templates) {
        const instance = documents.value.find(doc => 
          doc.template_id === template.templateId
        )
        
        allDocs.push({
          ...template,
          instance
        })
      }
    }
  }
  
  return allDocs.sort((a, b) => a.title.localeCompare(b.title))
}

// API calls
const fetchDocuments = async () => {
  const commandMap = {
    campaign: 'get_campaign_documents',
    module: 'get_module_documents',
    session: 'get_session_documents'
  }
  
  const command = commandMap[props.entityType]
  const params = props.entityType === 'campaign' 
    ? { campaignId: props.entityId }
    : { moduleId: props.entityId }
  
  const data = await loadDocuments(command, params)
  if (data) {
    documents.value = data
  }
}

const handleDocumentClick = async (doc: any) => {
  debugDocument('click', { doc, stage: getDocumentStage(doc.templateId) })
  const stage = getDocumentStage(doc.templateId)
  
  if (!isStageAccessible(stage)) {
    debugDocument('stage-locked', { stage })
    return
  }
  
  if (!doc.instance) {
    // Create document instance logic here
    const simpleDoc = await createDocumentInstance(doc)
    if (simpleDoc) {
      selectedDocument.value = simpleDoc
      emit('selectDocument', simpleDoc)
    }
  } else {
    selectedDocument.value = doc.instance
    emit('selectDocument', doc.instance)
  }
}

const createDocumentInstance = async (doc: any): Promise<Document | null> => {
  // This would be implemented based on entity type
  // For now, return null
  return null
}

const toggleDocumentCompletion = async (doc: any) => {
  if (!doc.instance) return
  
  const command = doc.instance.completed_at 
    ? 'mark_document_incomplete' 
    : 'mark_document_complete'
  
  try {
    await invoke(command, {
      documentId: doc.instance.id
    })
    
    await fetchDocuments()
    emit('documentCompletionChanged', doc.instance)
  } catch (e) {
    console.error('Failed to toggle document completion:', e)
  }
}

// Lifecycle
onMounted(() => {
  fetchDocuments()
})

watch(() => props.entityId, () => {
  fetchDocuments()
})

// Expose for parent components if needed
defineExpose({
  refresh: fetchDocuments,
  selectedDocument
})
</script>

<style scoped>
.document-sidebar {
  width: 320px;
  background-color: var(--color-surface-variant);
  border-right: 1px solid var(--color-border);
  display: flex;
  flex-direction: column;
  height: 100%;
}

.sidebar-header {
  padding: 1.5rem 1.5rem 1rem;
  border-bottom: 1px solid var(--color-border);
}

.sidebar-header h3 {
  margin: 0;
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--color-text-primary);
}

.document-content {
  flex: 1;
  overflow-y: auto;
  padding: 1rem;
}

.loading-state {
  text-align: center;
  padding: 2rem;
  color: var(--color-text-secondary);
}

.stage-group {
  margin-bottom: 1.5rem;
}

.stage-header {
  margin-bottom: 0.75rem;
}

.stage-header h4 {
  margin: 0 0 0.5rem 0;
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--color-text-secondary);
}

.progress-bar {
  height: 4px;
  background-color: var(--color-surface);
  border-radius: 2px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background-color: var(--color-success);
  transition: width 0.3s ease;
}

.document-items {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.document-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.625rem 0.75rem;
  background-color: var(--color-surface);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s ease;
  border: 1px solid transparent;
}

.document-item:hover {
  background-color: var(--color-surface-hover);
  transform: translateX(2px);
}

.document-item.selected {
  background-color: var(--color-primary-surface);
  border-color: var(--color-primary);
}

.document-item.completed {
  opacity: 0.7;
  background-color: var(--color-surface);
}

.document-item.locked {
  opacity: 0.5;
  cursor: not-allowed;
}

.document-item.locked:hover {
  transform: none;
}

.document-icon {
  width: 16px;
  height: 16px;
  flex-shrink: 0;
  cursor: pointer;
  transition: transform 0.2s ease;
}

.document-icon:hover:not(.locked) {
  transform: scale(1.1);
}

.document-icon.locked {
  cursor: not-allowed;
}

.document-title {
  flex: 1;
  font-size: 0.875rem;
  color: var(--color-text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.document-title.optional {
  color: var(--color-text-secondary);
}

.optional-label {
  font-size: 0.75rem;
  color: var(--color-text-tertiary);
  margin-left: 0.25rem;
}

.completion-checkbox {
  width: 20px;
  height: 20px;
  border: 2px solid var(--color-border);
  border-radius: 4px;
  background-color: var(--color-surface);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  flex-shrink: 0;
}

.completion-checkbox:hover {
  border-color: var(--color-success);
  background-color: var(--color-success-surface);
}

.completion-checkbox.checked {
  background-color: var(--color-success);
  border-color: var(--color-success);
}

.completion-checkbox.checked span {
  color: white;
  font-weight: bold;
  font-size: 0.75rem;
}

.active-documents .document-items {
  padding: 0.5rem 0;
}
</style>