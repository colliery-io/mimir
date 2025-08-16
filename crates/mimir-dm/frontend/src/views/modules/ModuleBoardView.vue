<template>
  <MainLayout>
    <div class="module-board-container">
      <!-- Document Sidebar -->
      <ModuleDocumentSidebar 
        v-if="module && boardConfig"
        ref="documentSidebar"
        :module-id="module.id"
        :module-stage="module.status"
        :board-config="boardConfig"
        @select-document="handleSelectDocument"
        @create-document="handleCreateDocument"
        @document-completion-changed="handleDocumentCompletionChanged"
      />
      
      <!-- Main Board Content -->
      <div class="module-board">
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
          <ModuleStageLandingView 
            v-if="!selectedDocument && module && boardConfig"
            :stage="currentStage"
            :documents="documents"
            :module="module"
            :boardConfig="boardConfig"
            :campaign="campaign"
            @create-document="handleCreateDocumentFromTemplate"
            @edit-document="handleEditDocument" 
            @transition-stage="handleTransitionStage"
            @open-session-document="handleEditDocument"
          />
          
          <!-- Document Editor (when document selected) -->
          <DocumentEditor 
            v-else-if="selectedDocument && module"
            :document="selectedDocument"
            :campaign-id="module.campaign_id"
            :module-id="module.id"
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
import { useRoute } from 'vue-router'
import MainLayout from '../../components/layout/MainLayout.vue'
import ModuleDocumentSidebar from '../../components/modules/ModuleDocumentSidebar.vue'
import ModuleStageLandingView from '../../components/modules/ModuleStageLandingView.vue'
import DocumentEditor from '../../components/campaigns/DocumentEditor.vue'

const route = useRoute()
const moduleId = computed(() => parseInt(route.params.id as string))

// Types
interface Module {
  id: number
  campaign_id: number
  name: string
  module_number: number
  status: string
  expected_sessions: number
  actual_sessions: number
  created_at: string
  started_at: string | null
  completed_at: string | null
}

interface Document {
  id: number
  campaign_id: number
  module_id: number | null
  template_id: string
  document_type: string
  title: string
  file_path: string
  completed_at: string | null
}

// Local state
const module = ref<Module | null>(null)
const loading = ref(false)
const error = ref<string | null>(null)
const selectedDocument = ref<Document | null>(null)
const documents = ref<Document[]>([])
const boardConfig = ref<any>(null)
const campaign = ref<any>(null)
const documentSidebar = ref<any>(null)

// Dynamic stages from board configuration
const stages = computed(() => {
  if (!boardConfig.value) return []
  return boardConfig.value.stages.map((stage: any) => ({
    key: stage.key,
    name: stage.display_name.toUpperCase()
  }))
})

// Current stage
const currentStage = computed(() => {
  return module.value?.status || 'planning'
})

// Check if a stage is completed (before the current stage)
const isStageCompleted = (stageKey: string): boolean => {
  if (!boardConfig.value || !module.value) return false
  const stageOrder = boardConfig.value.stages.map((s: any) => s.key)
  const currentIndex = stageOrder.indexOf(currentStage.value)
  const checkIndex = stageOrder.indexOf(stageKey)
  return checkIndex < currentIndex
}

// Get next stage info
const nextStage = computed(() => {
  if (!boardConfig.value || !module.value) return null
  const stageOrder = boardConfig.value.stages.map((s: any) => s.key)
  const currentIndex = stageOrder.indexOf(currentStage.value)
  if (currentIndex === -1 || currentIndex >= stageOrder.length - 1) return null
  return boardConfig.value.stages[currentIndex + 1]
})

const nextStageName = computed(() => {
  return nextStage.value?.display_name || 'Next Stage'
})

// Check if can progress to next stage
const canProgressToNext = computed(() => {
  if (!boardConfig.value || !module.value || !nextStage.value) return false
  if (module.value.status === 'completed') return false
  
  // Get current stage metadata
  const currentStageConfig = boardConfig.value.stages.find((s: any) => s.key === currentStage.value)
  if (!currentStageConfig) return false
  
  // Get required documents for current stage
  const requiredDocs = currentStageConfig.required_documents || []
  
  // Check which documents don't require completion
  const noCompletionDocs = currentStageConfig.no_completion_required_documents || []
  
  // Filter to only documents that need completion
  const completionRequiredDocs = requiredDocs.filter((docId: string) => 
    !noCompletionDocs.includes(docId)
  )
  
  // Check if all required documents that need completion are complete
  const completedDocs = documents.value.filter(doc => 
    completionRequiredDocs.includes(doc.template_id) && doc.completed_at
  )
  
  return completedDocs.length === completionRequiredDocs.length && completionRequiredDocs.length > 0
})

// Proceed to next stage
// Load board configuration
const loadBoardConfiguration = async () => {
  try {
    const response = await invoke<{ data: any }>('get_board_configuration', {
      boardType: 'module'
    })
    boardConfig.value = response.data
    console.log('Loaded module board configuration:', boardConfig.value)
  } catch (e) {
    console.error('Failed to load board configuration:', e)
  }
}

// Load module data
const loadModule = async () => {
  loading.value = true
  error.value = null
  
  try {
    // Load board configuration first
    await loadBoardConfiguration()
    
    const response = await invoke<{ data: Module }>('get_module', { 
      id: moduleId.value 
    })
    module.value = response.data
    console.log('Loaded module:', module.value)
    
    // Load campaign info
    await loadCampaign()
    
    // Initialize stage documents if needed
    await initializeStageDocuments()
    
    // Load existing documents
    await loadDocuments()
  } catch (e) {
    console.error('Failed to load module:', e)
    error.value = 'Failed to load module'
  } finally {
    loading.value = false
  }
}

// Load campaign info for directory path
const loadCampaign = async () => {
  if (!module.value) return
  
  try {
    const response = await invoke<{ data: any }>('get_campaign', {
      id: module.value.campaign_id
    })
    campaign.value = response.data
  } catch (e) {
    console.error('Failed to load campaign:', e)
  }
}

// Initialize documents for the current stage
const initializeStageDocuments = async () => {
  if (!module.value || !campaign.value) return
  
  try {
    const response = await invoke<{ data: string[] }>('initialize_module_documents', {
      request: {
        module_id: moduleId.value,
        campaign_directory: campaign.value.directory_path
      }
    })
    
    if (response.data && response.data.length > 0) {
      console.log('Initialized module documents:', response.data)
      // Reload documents after initialization
      await loadDocuments()
      
      // Force sidebar to reload documents too
      if (documentSidebar.value?.loadDocuments) {
        await documentSidebar.value.loadDocuments()
      }
    }
  } catch (e) {
    console.error('Failed to initialize module documents:', e)
  }
}

// Load all documents for the module
const loadDocuments = async () => {
  try {
    const response = await invoke<{ data: Document[] }>('get_module_documents', {
      request: {
        module_id: moduleId.value
      }
    })
    documents.value = response.data || []
  } catch (e) {
    console.error('Failed to load documents:', e)
  }
}

// Handle document selection from sidebar
const handleSelectDocument = (document: Document) => {
  selectedDocument.value = document
}

// Handle create document from sidebar
const handleCreateDocument = () => {
  console.log('Create new document')
  // TODO: Open document creation dialog
}

// Handle create document from template (from StageLandingView)
const handleCreateDocumentFromTemplate = async (templateId: string) => {
  if (!module.value || !campaign.value) return
  
  try {
    const response = await invoke<{ data: Document }>('create_document_from_template', {
      campaignId: module.value.campaign_id,
      moduleId: moduleId.value,
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
const handleEditDocument = (document: Document) => {
  selectedDocument.value = document
}

// Handle stage transition
const handleTransitionStage = async (newStage: string) => {
  try {
    const response = await invoke<{ data: Module }>('transition_module_stage', {
      request: {
        module_id: moduleId.value,
        new_stage: newStage
      }
    })
    
    if (response.data) {
      module.value = response.data
      
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
const handleDocumentUpdated = (updatedDocument: Document) => {
  // Update the document in our local list
  const index = documents.value.findIndex(d => d.id === updatedDocument.id)
  if (index !== -1) {
    documents.value[index] = updatedDocument
  }
}

// Handle document completion changed from sidebar
const handleDocumentCompletionChanged = (updatedDocument: Document) => {
  // Update the document in our local list
  const index = documents.value.findIndex(d => 
    d.id === updatedDocument.id || 
    (d.template_id === updatedDocument.template_id && d.module_id === updatedDocument.module_id)
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
const handleStageTransitioned = async (updatedModule: Module) => {
  // Update the module with proper reactivity
  if (module.value) {
    module.value = { ...module.value, status: updatedModule.status }
  }
  
  // Initialize documents for the new stage
  await initializeStageDocuments()
  
  // Reload documents for the new stage
  await loadDocuments()
  
  // Force sidebar to reload documents
  if (documentSidebar.value?.loadDocuments) {
    await documentSidebar.value.loadDocuments()
  }
  
  // Clear document selection to show landing page
  selectedDocument.value = null
}

// Watch for module ID changes (when navigating between modules)
watch(() => route.params.id, (newId, oldId) => {
  if (newId !== oldId && newId) {
    // Clear current state
    selectedDocument.value = null
    documents.value = []
    
    // Reload module data
    loadModule()
  }
})

onMounted(() => {
  loadModule()
})
</script>

<style scoped>
/* Module-specific container styles */
.module-board-container {
  display: flex;
  height: 100%;
  overflow: hidden;
}

.module-board {
  flex: 1;
  padding: var(--spacing-lg);
  overflow-y: auto;
}

/* Stage Transition Prompt */
</style>