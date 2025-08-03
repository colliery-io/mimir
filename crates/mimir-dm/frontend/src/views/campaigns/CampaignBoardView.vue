<template>
  <MainLayout>
    <div class="campaign-board-container">
      <!-- Document Sidebar -->
      <DocumentSidebar 
        v-if="campaign"
        :campaign-id="campaign.id"
        :campaign-stage="campaign.status" 
        @select-document="handleSelectDocument"
        @create-document="handleCreateDocument"
      />
      
      <!-- Main Board Content -->
      <div class="campaign-board">
        <!-- Kanban Stage Progress -->
        <div class="stage-progress">
          <div 
            v-for="(stage, index) in stages" 
            :key="stage.key"
            class="stage-indicator"
            :class="{ 
              active: currentStage === stage.key,
              completed: isStageCompleted(stage.key)
            }"
          >
            <div class="stage-name">{{ stage.name }}</div>
            <div class="stage-marker" v-if="currentStage === stage.key">●</div>
            <div class="stage-arrow-point"></div>
          </div>
        </div>

        <!-- Main Content Area -->
        <div class="main-content">
          <!-- Stage Landing View (default) -->
          <StageLandingView 
            v-if="!selectedDocument && campaign"
            :stage="currentStage"
            :documents="documents"
            :campaign="campaign"
            @create-document="handleCreateDocumentFromTemplate"
            @edit-document="handleEditDocument" 
            @transition-stage="handleTransitionStage"
          />
          
          <!-- Document Editor (when document selected) -->
          <DocumentEditor 
            v-else-if="selectedDocument"
            :document="selectedDocument"
            :campaign-id="parseInt(id)"
            @close="selectedDocument = null"
            @updated="handleDocumentUpdated"
          />
        </div>
      </div>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import MainLayout from '../../components/layout/MainLayout.vue'
import DocumentSidebar from '../../components/campaigns/DocumentSidebar.vue'
import StageLandingView from '../../components/campaigns/StageLandingView.vue'
import DocumentEditor from '../../components/campaigns/DocumentEditor.vue'
import type { Campaign } from '../../types/campaign'

const props = defineProps<{
  id: string
}>()

// Local state
const campaign = ref<Campaign | null>(null)
const loading = ref(false)
const error = ref<string | null>(null)
const selectedDocument = ref<any>(null)
const documents = ref<any[]>([])

// Stage definitions
const stages = [
  { key: 'concept', name: 'CONCEPT' },
  { key: 'session_zero', name: 'SESSION ZERO' },
  { key: 'integration', name: 'INTEGRATION' },
  { key: 'active', name: 'ACTIVE' }
]

// Map status to stage for display
const currentStage = computed(() => {
  if (!campaign.value) return 'concept'
  
  // Map planning status to concept stage
  if (campaign.value.status === 'planning') {
    return 'concept'
  }
  
  // Otherwise use status directly as the stage
  return campaign.value.status
})

// Check if a stage is completed (before the current stage)
const isStageCompleted = (stageKey: string): boolean => {
  const stageOrder = ['concept', 'session_zero', 'integration', 'active']
  const currentIndex = stageOrder.indexOf(currentStage.value)
  const checkIndex = stageOrder.indexOf(stageKey)
  return checkIndex < currentIndex
}

// Load campaign data
const loadCampaign = async () => {
  loading.value = true
  error.value = null
  
  try {
    const response = await invoke<{ data: Campaign }>('get_campaign', { 
      id: parseInt(props.id) 
    })
    campaign.value = response.data
    console.log('Loaded campaign:', campaign.value)
    
    // Initialize stage documents if this is the first time
    await initializeStageDocuments()
    
    // Load existing documents
    await loadDocuments()
  } catch (e) {
    console.error('Failed to load campaign:', e)
    error.value = 'Failed to load campaign'
  } finally {
    loading.value = false
  }
}

// Initialize documents for the current stage
const initializeStageDocuments = async () => {
  try {
    const response = await invoke<{ data: string[] }>('initialize_stage_documents', {
      campaignId: parseInt(props.id)
    })
    
    if (response.data && response.data.length > 0) {
      console.log('Initialized documents:', response.data)
      // Reload documents after initialization
      await loadDocuments()
    }
  } catch (e) {
    console.error('Failed to initialize stage documents:', e)
  }
}

// Load all documents for the campaign
const loadDocuments = async () => {
  try {
    const response = await invoke<{ data: any[] }>('get_campaign_documents', {
      campaignId: parseInt(props.id)
    })
    documents.value = response.data || []
  } catch (e) {
    console.error('Failed to load documents:', e)
  }
}

// Handle document selection from sidebar
const handleSelectDocument = (document: any) => {
  selectedDocument.value = document
}

// Handle create document from sidebar
const handleCreateDocument = () => {
  console.log('Create new document')
  // TODO: Open document creation dialog
}

// Handle create document from template (from StageLandingView)
const handleCreateDocumentFromTemplate = async (templateId: string) => {
  try {
    const response = await invoke<{ data: any }>('create_document_from_template', {
      campaignId: parseInt(props.id),
      templateId: templateId
    })
    
    if (response.data) {
      documents.value.push(response.data)
      selectedDocument.value = response.data
    }
  } catch (e) {
    console.error('Failed to create document:', e)
  }
}

// Handle edit document (from StageLandingView)
const handleEditDocument = (document: any) => {
  selectedDocument.value = document
}

// Handle stage transition
const handleTransitionStage = async (newStage: string) => {
  try {
    const response = await invoke<{ data: Campaign }>('transition_campaign_stage', {
      campaignId: parseInt(props.id),
      newStage: newStage
    })
    
    if (response.data) {
      campaign.value = response.data
      // Reload documents for new stage
      await loadDocuments()
    }
  } catch (e) {
    console.error('Failed to transition stage:', e)
  }
}

// Handle document updated (e.g., marked as complete)
const handleDocumentUpdated = (updatedDocument: any) => {
  // Update the document in our local list
  const index = documents.value.findIndex(d => d.id === updatedDocument.id)
  if (index !== -1) {
    documents.value[index] = updatedDocument
  }
}

onMounted(() => {
  loadCampaign()
})
</script>

<style scoped>
.campaign-board-container {
  display: flex;
  height: 100%;
  overflow: hidden;
}

.campaign-board {
  flex: 1;
  padding: var(--spacing-lg);
  overflow-y: auto;
}

/* Stage Progress */
.stage-progress {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-xl) var(--spacing-lg);
  background-color: var(--color-surface);
  border-radius: var(--radius-lg);
  margin-bottom: var(--spacing-xl);
  gap: 0;
}

.stage-indicator {
  background-color: var(--color-surface-variant);
  border: 2px solid var(--color-border);
  padding: var(--spacing-sm) var(--spacing-xl) var(--spacing-sm) var(--spacing-lg);
  position: relative;
  transition: all var(--transition-base);
  margin-right: -2px; /* Overlap borders */
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}

/* First stage has rounded left corners */
.stage-indicator:first-child {
  border-radius: var(--radius-md) 0 0 var(--radius-md);
}

/* Last stage has different styling */
.stage-indicator:last-child {
  margin-right: 0;
}

.stage-indicator:last-child .stage-arrow-point {
  display: none;
}

/* Arrow point on the right */
.stage-arrow-point {
  position: absolute;
  right: -20px;
  top: 50%;
  transform: translateY(-50%);
  width: 0;
  height: 0;
  border-left: 20px solid var(--color-border);
  border-top: 20px solid transparent;
  border-bottom: 20px solid transparent;
  z-index: 3;
}

.stage-arrow-point::before {
  content: '';
  position: absolute;
  right: 2px;
  top: -18px;
  width: 0;
  height: 0;
  border-left: 18px solid var(--color-surface-variant);
  border-top: 18px solid transparent;
  border-bottom: 18px solid transparent;
}

/* Completed stages - only border changes */
.stage-indicator.completed {
  border-color: var(--color-success);
}

.stage-indicator.completed .stage-arrow-point {
  border-left-color: var(--color-success);
}

/* Active stage - only border changes */
.stage-indicator.active {
  border-color: var(--color-primary-500);
}

.stage-indicator.active .stage-arrow-point {
  border-left-color: var(--color-primary-500);
}

.stage-name {
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--color-text-secondary);
  z-index: 2;
}

.stage-indicator.active .stage-name,
.stage-indicator.completed .stage-name {
  color: var(--color-text);
}

.stage-marker {
  text-align: center;
  color: var(--color-primary-600);
  font-size: 1.25rem;
  z-index: 2;
}

/* Main Content Area */
.main-content {
  background-color: var(--color-surface);
  border-radius: var(--radius-lg);
  min-height: calc(100vh - 300px);
  overflow-y: auto;
}
</style>