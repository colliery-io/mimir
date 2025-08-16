<template>
  <MainLayout>
    <div class="campaign-board-container">
      <!-- Document Sidebar -->
      <DocumentSidebar 
        v-if="campaign && boardConfig"
        :campaign-id="campaign.id"
        :campaign-stage="campaign.status"
        :board-config="boardConfig"
        @select-document="handleSelectDocument"
        @create-document="handleCreateDocument"
        @document-completion-changed="handleDocumentCompletionChanged"
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
            :style="{ zIndex: stages.length - index }"
          >
            <div class="stage-content">
              <div class="stage-name">{{ stage.name }}</div>
              <div class="stage-marker" v-if="currentStage === stage.key">●</div>
            </div>
            <div class="stage-arrow-point"></div>
          </div>
        </div>

        <!-- Main Content Area -->
        <div class="main-content">
          <!-- Stage Landing View (default) -->
          <StageLandingView 
            v-if="!selectedDocument && campaign && boardConfig"
            :stage="currentStage"
            :documents="documents"
            :campaign="campaign"
            :boardConfig="boardConfig"
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
            @stage-transitioned="handleStageTransitioned"
          />
        </div>
      </div>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
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
const boardConfig = ref<any>(null)

// Dynamic stages from board configuration
const stages = computed(() => {
  if (!boardConfig.value) return []
  return boardConfig.value.stages.map((stage: any) => ({
    key: stage.key,
    name: stage.display_name.toUpperCase()
  }))
})

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
  if (!boardConfig.value) return false
  const stageOrder = boardConfig.value.stages.map((s: any) => s.key)
  const currentIndex = stageOrder.indexOf(currentStage.value)
  const checkIndex = stageOrder.indexOf(stageKey)
  return checkIndex < currentIndex
}

// Load board configuration
const loadBoardConfiguration = async () => {
  try {
    const response = await invoke<{ data: any }>('get_board_configuration', {
      boardType: 'campaign'
    })
    boardConfig.value = response.data
    console.log('Loaded board configuration:', boardConfig.value)
  } catch (e) {
    console.error('Failed to load board configuration:', e)
  }
}

// Load campaign data
const loadCampaign = async () => {
  loading.value = true
  error.value = null
  
  try {
    // Load board configuration first
    await loadBoardConfiguration()
    
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
      
      // Initialize documents for the new stage
      await initializeStageDocuments()
      
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

// Handle document completion changed from sidebar
const handleDocumentCompletionChanged = (updatedDocument: any) => {
  // Update the document in our local list
  const index = documents.value.findIndex(d => 
    d.id === updatedDocument.id || 
    (d.template_id === updatedDocument.template_id && d.campaign_id === updatedDocument.campaign_id)
  )
  if (index !== -1) {
    documents.value[index] = updatedDocument
  } else {
    // If not found, add it (for temporary documents)
    documents.value.push(updatedDocument)
  }
  
  // Force reactivity update for the landing view
  documents.value = [...documents.value]
}

// Handle stage transition
const handleStageTransitioned = (updatedCampaign: any) => {
  // Update the campaign status
  if (campaign.value) {
    campaign.value.status = updatedCampaign.status
  }
  
  // Reload documents for the new stage
  loadDocuments()
  
  // Clear document selection to show landing page
  selectedDocument.value = null
}

// Watch for campaign ID changes (when switching campaigns via dropdown)
watch(() => props.id, (newId, oldId) => {
  if (newId !== oldId) {
    // Clear current state
    selectedDocument.value = null
    documents.value = []
    
    // Reload campaign data
    loadCampaign()
  }
})

onMounted(() => {
  loadCampaign()
})
</script>

<style scoped>
/* Campaign-specific overrides */
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

/* Campaign boards have slightly wider stage indicators */
.stage-indicator {
  max-width: 200px;
  min-width: 120px;
}
</style>