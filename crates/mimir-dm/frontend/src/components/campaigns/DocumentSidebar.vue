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
        <!-- Concept Stage Documents -->
        <div class="stage-group">
          <div class="stage-header">
            <h4>Concept Stage ({{ conceptDocs.completed }}/{{ conceptDocs.total }})</h4>
            <div class="progress-bar">
              <div 
                class="progress-fill" 
                :style="{ width: conceptDocs.percentage + '%' }"
              ></div>
            </div>
          </div>
          <div class="document-items">
            <div 
              v-for="doc in conceptDocs.documents" 
              :key="doc.templateId"
              class="document-item"
              :class="{ 
                selected: selectedDocument?.template_id === doc.templateId,
                completed: doc.instance?.completed_at,
                locked: !isStageAccessible('concept')
              }"
              @click="handleDocumentClick(doc)"
            >
              <img 
                v-if="getDocumentIcon(doc)" 
                :src="getDocumentIcon(doc)" 
                :alt="doc.instance ? 'Edit' : 'Locked'"
                class="document-icon"
              />
              <span v-else-if="doc.instance?.completed_at" class="document-icon-text">✓</span>
              <span v-else class="document-icon-placeholder"></span>
              <span class="document-title">{{ doc.title }}</span>
            </div>
          </div>
        </div>

        <!-- Session Zero Stage Documents -->
        <div class="stage-group">
          <div class="stage-header">
            <h4>Session Zero ({{ sessionZeroDocs.completed }}/{{ sessionZeroDocs.total }})</h4>
            <div class="progress-bar">
              <div 
                class="progress-fill" 
                :style="{ width: sessionZeroDocs.percentage + '%' }"
              ></div>
            </div>
          </div>
          <div class="document-items">
            <div 
              v-for="doc in sessionZeroDocs.documents" 
              :key="doc.templateId"
              class="document-item"
              :class="{ 
                selected: selectedDocument?.template_id === doc.templateId,
                completed: doc.instance?.completed_at,
                locked: !isStageAccessible('session_zero')
              }"
              @click="handleDocumentClick(doc)"
            >
              <img 
                v-if="getDocumentIcon(doc)" 
                :src="getDocumentIcon(doc)" 
                :alt="doc.instance ? 'Edit' : 'Locked'"
                class="document-icon"
              />
              <span v-else-if="doc.instance?.completed_at" class="document-icon-text">✓</span>
              <span v-else class="document-icon-placeholder"></span>
              <span class="document-title">{{ doc.title }}</span>
            </div>
          </div>
        </div>

        <!-- Integration Stage Documents -->
        <div class="stage-group">
          <div class="stage-header">
            <h4>Integration ({{ integrationDocs.completed }}/{{ integrationDocs.total }})</h4>
            <div class="progress-bar">
              <div 
                class="progress-fill" 
                :style="{ width: integrationDocs.percentage + '%' }"
              ></div>
            </div>
          </div>
          <div class="document-items">
            <div 
              v-for="doc in integrationDocs.documents" 
              :key="doc.templateId"
              class="document-item"
              :class="{ 
                selected: selectedDocument?.template_id === doc.templateId,
                completed: doc.instance?.completed_at,
                locked: !isStageAccessible('integration')
              }"
              @click="handleDocumentClick(doc)"
            >
              <img 
                v-if="getDocumentIcon(doc)" 
                :src="getDocumentIcon(doc)" 
                :alt="doc.instance ? 'Edit' : 'Locked'"
                class="document-icon"
              />
              <span v-else-if="doc.instance?.completed_at" class="document-icon-text">✓</span>
              <span v-else class="document-icon-placeholder"></span>
              <span class="document-title">{{ doc.title }}</span>
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
  campaignId: number
  campaignStage: string
}>()

const emit = defineEmits<{
  selectDocument: [document: Document]
  createDocument: []
}>()

// Define document templates for each stage
const stageDocuments = {
  concept: [
    { templateId: 'campaign-sparks', title: 'Campaign Sparks' },
    { templateId: 'campaign-pitch', title: 'Campaign Pitch' },
    { templateId: 'big-three', title: 'Big Three' },
    { templateId: 'first-adventure', title: 'First Adventure' }
  ],
  session_zero: [
    { templateId: 'starting-scenario', title: 'Starting Scenario' },
    { templateId: 'world-primer', title: 'World Primer' },
    { templateId: 'character-guidelines', title: 'Character Guidelines' },
    { templateId: 'table-expectations', title: 'Table Expectations' },
    { templateId: 'character-integration', title: 'Character Integration Forms' },
    { templateId: 'session-zero-packet', title: 'Session Zero Packet' }
  ],
  integration: [
    { templateId: 'campaign-bible', title: 'Campaign Bible' },
    { templateId: 'character-integration-notes', title: 'Character Integration Notes' },
    { templateId: 'major-npcs', title: 'Major NPCs' },
    { templateId: 'world-timeline', title: 'World Events Timeline' }
  ]
}

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

// Computed properties for stage document groups
const conceptDocs = computed(() => getStageDocuments('concept'))
const sessionZeroDocs = computed(() => getStageDocuments('session_zero'))
const integrationDocs = computed(() => getStageDocuments('integration'))

// Get documents for a specific stage
const getStageDocuments = (stage: string) => {
  const templates = stageDocuments[stage as keyof typeof stageDocuments] || []
  const stageDocumentList = templates.map(template => {
    const instance = documents.value.find(doc => doc.template_id === template.templateId)
    return {
      ...template,
      instance
    }
  })
  
  const completed = stageDocumentList.filter(doc => doc.instance?.completed_at).length
  const total = stageDocumentList.length
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
  const stageOrder = ['concept', 'session_zero', 'integration', 'active']
  const currentIndex = stageOrder.indexOf(props.campaignStage)
  const checkIndex = stageOrder.indexOf(stage)
  return checkIndex <= currentIndex
}

// Get document icon based on state
const getDocumentIcon = (doc: any): string | undefined => {
  const theme = themeStore.currentTheme as 'light' | 'dark' | 'hyper'
  
  if (!isStageAccessible(getDocumentStage(doc.templateId))) {
    return iconMap[theme]?.locked
  }
  if (doc.instance?.completed_at) {
    // For completed, we'll use a check mark or just hide the icon
    return undefined
  }
  if (doc.instance) {
    return iconMap[theme]?.edit
  }
  return undefined // No icon for not started
}

// Get which stage a template belongs to
const getDocumentStage = (templateId: string): string => {
  for (const [stage, docs] of Object.entries(stageDocuments)) {
    if (docs.some(d => d.templateId === templateId)) {
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
  const stage = getDocumentStage(doc.templateId)
  
  // Check if stage is accessible
  if (!isStageAccessible(stage)) {
    console.log('Stage is locked:', stage)
    return
  }
  
  // If document doesn't exist, create it
  if (!doc.instance) {
    await createDocument(doc.templateId, doc.title)
  } else {
    selectDocument(doc.instance)
  }
}

// Create a new document from template
const createDocument = async (templateId: string, title: string) => {
  try {
    const response = await invoke<{ data: Document }>('create_document', {
      newDocument: {
        campaign_id: props.campaignId,
        module_id: null,
        session_id: null,
        template_id: templateId,
        document_type: templateId.replace('-', '_'),
        title: title,
        file_path: `${props.campaignId}/${templateId}.md`
      }
    })
    
    if (response.data) {
      documents.value.push(response.data)
      selectDocument(response.data)
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

// Watch for campaign or stage changes
watch([() => props.campaignId, () => props.campaignStage], () => {
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
  font-size: 1rem;
  font-weight: 600;
  color: var(--color-text);
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
}

.document-item:hover:not(.locked) {
  background-color: var(--color-surface-variant);
}

.document-item.selected {
  background-color: var(--color-primary-100);
  color: var(--color-primary-700);
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
}

</style>