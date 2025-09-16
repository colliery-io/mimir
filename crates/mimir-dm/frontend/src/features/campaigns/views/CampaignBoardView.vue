<template>
  <BaseBoardView
    entity-type="campaign"
    :entity="campaign"
    :board-config="boardConfig"
    :completed-stages="completedStages"
  >
    <!-- Document Sidebar -->
    <template #sidebar>
      <DocumentSidebar 
        v-if="campaign && boardConfig"
        :campaign-id="campaign.id"
        :campaign-stage="campaign.status"
        :board-config="boardConfig"
        @select-document="handleSelectDocument"
        @create-document="handleCreateDocument"
        @document-completion-changed="handleDocumentCompletionChanged"
      />
    </template>
    
    <!-- Main Content -->
    <template #content>
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
    </template>
  </BaseBoardView>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { DocumentService } from '@/services/DocumentService'
import { useSharedContextStore } from '@/stores/sharedContext'
import BaseBoardView from '../../../shared/components/ui/BaseBoardView.vue'
import DocumentSidebar from '../components/DocumentSidebar.vue'
import StageLandingView from '../components/StageLandingView.vue'
import DocumentEditor from '../components/DocumentEditor.vue'
import { useStageProgress } from '../../../shared/composables/useStageProgress'
import { useApiCall } from '../../../shared/composables/useApiCall'
import type { Campaign, BoardConfig } from '../../../types'

const props = defineProps<{
  id: string
}>()

const contextStore = useSharedContextStore()

// Local state
const campaign = ref<Campaign | null>(null)
const selectedDocument = ref<any>(null)
const documents = ref<any[]>([])
const boardConfig = ref<BoardConfig | null>(null)

// Use composables
const { loading: boardLoading, error: boardError, execute: loadBoardApi } = useApiCall<BoardConfig>()
const { loading: campaignLoading, error: campaignError, execute: loadCampaignApi } = useApiCall<Campaign>()
const campaignComputed = computed(() => campaign.value)
const boardConfigComputed = computed(() => boardConfig.value)
const { currentStage, isStageCompleted, getStageIndex } = useStageProgress(
  campaignComputed,
  boardConfigComputed,
  'campaign'
)

// Completed stages for visual indication
const completedStages = computed(() => {
  if (!boardConfig.value || !boardConfig.value.stages) return []
  return boardConfig.value.stages
    .filter(stage => isStageCompleted(stage.key))
    .map(stage => stage.key)
})

// Load board configuration
const loadBoardConfiguration = async () => {
  const data = await loadBoardApi('get_board_configuration', {
    boardType: 'campaign'
  })
  if (data) {
    boardConfig.value = data
  }
}

// Load campaign data
const loadCampaign = async () => {
  // Load board configuration first
  await loadBoardConfiguration()
  
  const data = await loadCampaignApi('get_campaign', { 
    id: parseInt(props.id) 
  })
  
  if (data) {
    campaign.value = data
    
    // Update context with campaign info
    await contextStore.updateCampaign({
      id: campaign.value.id.toString(),
      name: campaign.value.name,
      currentStage: campaign.value.status || undefined,
      currentDocument: selectedDocument.value?.title || undefined,
      directory_path: campaign.value.directory_path || undefined
    })
    
    // Clear module/session context when on campaign board
    await contextStore.updateModule({})
    await contextStore.updateSession({})
    
    // Initialize stage documents if this is the first time
    await initializeStageDocuments()
    
    // Load existing documents
    await loadDocuments()
  }
}

// Initialize documents for the current stage
const initializeStageDocuments = async () => {
  try {
    const response = await invoke<{ data: string[] }>('initialize_stage_documents', {
      campaignId: parseInt(props.id)
    })
    
    if (response.data && response.data.length > 0) {
      // Reload documents after initialization
      await loadDocuments()
    }
  } catch (e) {
  }
}

// Load all documents for the campaign
const loadDocuments = async () => {
  try {
    documents.value = await DocumentService.list(undefined, parseInt(props.id))
  } catch (e) {
  }
}

// Handle document selection from sidebar
const handleSelectDocument = async (document: any) => {
  selectedDocument.value = document
  
  // Update context with current document
  if (document && campaign.value) {
    await contextStore.updateCampaign({
      id: campaign.value.id.toString(),
      name: campaign.value.name,
      currentStage: campaign.value.status || undefined,
      directory_path: campaign.value.directory_path || undefined,
      currentDocument: document.title || document.name
    })
  }
}

// Handle create document from sidebar
const handleCreateDocument = () => {
  // TODO: Open document creation dialog
}

// Handle create document from template (from StageLandingView)
const handleCreateDocumentFromTemplate = async (templateId: string) => {
  try {
    const newDoc = await DocumentService.create({
      title: templateId.replace(/[-_]/g, ' '),
      content: '',
      documentType: 'task',
      templateId: templateId,
      campaignId: parseInt(props.id)
    })
    
    if (newDoc) {
      documents.value.push(newDoc)
      selectedDocument.value = newDoc
    }
  } catch (e) {
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
/* Campaign-specific overrides if needed */
</style>