<template>
  <MainLayout>
    <div class="module-board-container">
      <!-- Document Sidebar -->
      <ModuleDocumentSidebar 
        v-if="module && boardConfig"
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
import { ref, computed, onMounted } from 'vue'
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
  return module.value?.status || 'backlog'
})

// Check if a stage is completed (before the current stage)
const isStageCompleted = (stageKey: string): boolean => {
  if (!boardConfig.value || !module.value) return false
  const stageOrder = boardConfig.value.stages.map((s: any) => s.key)
  const currentIndex = stageOrder.indexOf(currentStage.value)
  const checkIndex = stageOrder.indexOf(stageKey)
  return checkIndex < currentIndex
}

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
const handleStageTransitioned = (updatedModule: Module) => {
  // Update the module status
  if (module.value) {
    module.value.status = updatedModule.status
  }
  
  // Reload documents for the new stage
  loadDocuments()
  
  // Clear document selection to show landing page
  selectedDocument.value = null
}

onMounted(() => {
  loadModule()
})
</script>

<style scoped>
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

/* Stage Progress */
.stage-progress {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-md) var(--spacing-lg);
  background-color: var(--color-surface);
  border-radius: var(--radius-lg);
  margin-bottom: var(--spacing-xl);
  gap: 0;
  overflow: hidden;
}

.stage-indicator {
  background-color: var(--color-surface-variant);
  border: 2px solid var(--color-border);
  padding: 0;
  position: relative;
  transition: all var(--transition-base);
  margin-right: -2px; /* Overlap borders */
  display: flex;
  align-items: center;
  justify-content: center;
  height: 36px;
  flex: 1;
  max-width: 180px;
  min-width: 100px;
}

/* First stage has rounded left corners */
.stage-indicator:first-child {
  border-radius: var(--radius-md) 0 0 var(--radius-md);
}

/* Last stage has different styling */
.stage-indicator:last-child {
  margin-right: 0;
  border-radius: 0 var(--radius-md) var(--radius-md) 0;
}

.stage-indicator:last-child .stage-arrow-point {
  display: none;
}

/* Arrow point on the right */
.stage-arrow-point {
  position: absolute;
  right: -18px;
  top: 50%;
  transform: translateY(-50%);
  width: 0;
  height: 0;
  border-left: 18px solid var(--color-border);
  border-top: 18px solid transparent;
  border-bottom: 18px solid transparent;
  z-index: 3;
}

.stage-arrow-point::before {
  content: '';
  position: absolute;
  right: 2px;
  top: -16px;
  width: 0;
  height: 0;
  border-left: 16px solid var(--color-surface-variant);
  border-top: 16px solid transparent;
  border-bottom: 16px solid transparent;
}

/* Completed stages */
.stage-indicator.completed {
  border-color: var(--color-success);
}

.stage-indicator.completed .stage-arrow-point {
  border-left-color: var(--color-success);
}

/* Active stage */
.stage-indicator.active {
  border-color: var(--color-primary-500);
}

.stage-indicator.active .stage-arrow-point {
  border-left-color: var(--color-primary-500);
}

.stage-content {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: 0 var(--spacing-md);
  z-index: 2;
  position: relative;
}

.stage-name {
  font-size: 0.7rem;
  font-weight: 600;
  color: var(--color-text-secondary);
  white-space: nowrap;
  letter-spacing: 0.5px;
  text-transform: uppercase;
}

.stage-indicator.active .stage-name,
.stage-indicator.completed .stage-name {
  color: var(--color-text);
}

.stage-marker {
  color: var(--color-primary-600);
  font-size: 0.875rem;
  line-height: 1;
}

/* Main Content Area */
.main-content {
  background-color: var(--color-surface);
  border-radius: var(--radius-lg);
  min-height: calc(100vh - 300px);
  overflow-y: auto;
}
</style>