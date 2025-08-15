<template>
  <div class="stage-landing">
    <div class="stage-header">
      <h2>{{ stageMetadata?.display_name || stage }}</h2>
      <p class="stage-description">{{ stageMetadata?.description }}</p>
    </div>

    <!-- Help Text -->
    <div v-if="stageMetadata?.help_text" class="help-section">
      <div class="help-icon">💡</div>
      <div class="help-content">
        <p>{{ stageMetadata.help_text }}</p>
      </div>
    </div>

    <!-- Module Info -->
    <div class="module-info">
      <div class="info-card">
        <h3>Module #{{ module.module_number }}: {{ module.name }}</h3>
        <div class="info-grid">
          <div class="info-item">
            <span class="label">Expected Sessions:</span>
            <span class="value">{{ module.expected_sessions }}</span>
          </div>
          <div class="info-item">
            <span class="label">Actual Sessions:</span>
            <span class="value">{{ module.actual_sessions }}</span>
          </div>
          <div class="info-item" v-if="module.started_at">
            <span class="label">Started:</span>
            <span class="value">{{ formatDate(module.started_at) }}</span>
          </div>
          <div class="info-item" v-if="progressPercentage > 0">
            <span class="label">Progress:</span>
            <span class="value">{{ progressPercentage }}%</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Document Templates for current stage -->
    <div v-if="hasDocuments" class="documents-section">
      <h3>Stage Documents</h3>
      
      <!-- Required Documents -->
      <div v-if="requiredDocuments.length > 0" class="document-category">
        <h4>Required Documents</h4>
        <div class="document-grid">
          <div 
            v-for="doc in requiredDocuments" 
            :key="doc.templateId"
            class="document-card"
            :class="{ completed: doc.isCompleted }"
          >
            <div class="document-status">
              <span v-if="doc.exists && doc.isCompleted" class="status-icon">✅</span>
              <span v-else-if="doc.exists" class="status-icon">📝</span>
              <span v-else class="status-icon">📄</span>
            </div>
            <h4>
              {{ doc.title }}
              <span v-if="!doc.requiresCompletion" class="tracking-badge">(Tracking)</span>
            </h4>
            <p class="document-description">{{ doc.description }}</p>
            <div class="document-actions">
              <button 
                v-if="!doc.exists"
                @click="createDocument(doc.templateId)"
                class="btn btn-primary"
              >
                Create Document
              </button>
              <button 
                v-else
                @click="editDocument(doc)"
                class="btn btn-secondary"
              >
                Edit Document
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Optional Documents -->
      <div v-if="optionalDocuments.length > 0" class="document-category">
        <h4>Optional Documents</h4>
        <div class="document-grid">
          <div 
            v-for="doc in optionalDocuments" 
            :key="doc.templateId"
            class="document-card optional"
            :class="{ completed: doc.isCompleted }"
          >
            <div class="document-status">
              <span v-if="doc.exists && doc.isCompleted" class="status-icon">✅</span>
              <span v-else-if="doc.exists" class="status-icon">📝</span>
              <span v-else class="status-icon">📄</span>
            </div>
            <h4>
              {{ doc.title }}
              <span v-if="!doc.requiresCompletion" class="tracking-badge">(Tracking)</span>
            </h4>
            <p class="document-description">{{ doc.description }}</p>
            <div class="document-actions">
              <button 
                v-if="!doc.exists"
                @click="createDocument(doc.templateId)"
                class="btn btn-outline"
              >
                Create Optional
              </button>
              <button 
                v-else
                @click="editDocument(doc)"
                class="btn btn-secondary"
              >
                Edit Document
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Session Management for Active Stage -->
    <div v-if="stage === 'active'" class="session-section">
      <h3>Session Management</h3>
      <div class="session-info">
        <p>Sessions Completed: {{ module.actual_sessions }} / {{ module.expected_sessions }}</p>
        <button @click="createNewSession" class="btn btn-primary">
          Create New Session
        </button>
      </div>
      
      <!-- List existing sessions -->
      <div v-if="sessions.length > 0" class="sessions-list">
        <h4>Module Sessions</h4>
        <div class="session-grid">
          <div v-for="session in sessions" :key="session.id" class="session-card" :class="{ active: session.status === 'active' }">
            <div class="session-header">
              <h5>Session #{{ session.session_number }}</h5>
              <select 
                v-model="session.status" 
                @change="handleSessionStatusChange(session)"
                class="session-status-select"
                :class="`status-${session.status}`"
              >
                <option 
                  v-for="stage in getAvailableTransitions(session.status)" 
                  :key="stage.key"
                  :value="stage.key"
                  :selected="stage.key === session.status"
                >
                  {{ stage.display_name }}
                </option>
              </select>
            </div>
            <div class="session-documents">
              <button @click="openSessionDocument(session, 'outline')" class="doc-link-btn">
                📝 Outline
              </button>
              <button @click="openSessionDocument(session, 'notes')" class="doc-link-btn">
                📔 Notes
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Stage Transition -->
    <div class="transition-section">
      <div v-if="canProgressToNext" class="transition-card">
        <h3>Ready to Progress?</h3>
        <p v-if="stageMetadata?.completion_message">{{ stageMetadata.completion_message }}</p>
        <p v-if="stageMetadata?.transition_prompt" class="transition-prompt">
          {{ stageMetadata.transition_prompt }}
        </p>
        <button @click="handleTransition" class="btn btn-primary btn-large">
          Move to {{ nextStage?.display_name || 'Next Stage' }}
        </button>
      </div>
      
      <div v-else-if="isCompleted" class="completion-card">
        <h3>🎉 Module Complete!</h3>
        <p>This module has been completed. Great work!</p>
      </div>
      
      <div v-else class="requirements-card">
        <h3>Stage Requirements</h3>
        <p>Complete all required documents to progress to the next stage.</p>
        <ul class="requirements-list">
          <li v-for="doc in missingRequiredDocuments" :key="doc">
            ❌ {{ formatDocumentName(doc) }}
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onActivated, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const props = defineProps<{
  stage: string
  documents: any[]
  module: any
  boardConfig: any
  campaign?: any
}>()

const emit = defineEmits<{
  createDocument: [templateId: string]
  editDocument: [document: any]
  transitionStage: [newStage: string]
  documentsUpdated: []
  openSessionDocument: [session: any, docType: string]
}>()

// Session state
const sessions = ref<any[]>([])
const sessionBoardConfig = ref<any>(null)

// Load sessions and config when entering active stage or when module changes
watch([() => props.stage, () => props.module], async ([newStage, newModule]) => {
  console.log('Stage or module changed:', newStage, newModule?.id)
  if (newStage === 'active' && newModule) {
    console.log('Loading sessions for active stage')
    await loadSessionBoardConfig()
    await loadSessions()
  }
}, { immediate: true })

// Also load when component is mounted (for when navigating back)
onMounted(async () => {
  if (props.stage === 'active' && props.module) {
    console.log('Component mounted, loading sessions')
    await loadSessionBoardConfig()
    await loadSessions()
  }
})

// Load when component is reactivated (if using keep-alive)
onActivated(async () => {
  if (props.stage === 'active' && props.module) {
    console.log('Component activated, loading sessions')
    await loadSessions()
  }
})

// Load session board configuration
const loadSessionBoardConfig = async () => {
  try {
    const response = await invoke<{ data: any }>('get_session_board_config')
    sessionBoardConfig.value = response.data
    console.log('Loaded session board config:', sessionBoardConfig.value)
  } catch (e) {
    console.error('Failed to load session board config:', e)
  }
}

// Load sessions for the module
const loadSessions = async () => {
  if (!props.module) {
    console.log('No module available for loading sessions')
    return
  }
  
  console.log('Loading sessions for module:', props.module.id)
  
  try {
    const response = await invoke<{ data: any[] }>('list_module_sessions', {
      request: {
        module_id: props.module.id
      }
    })
    sessions.value = response.data || []
    console.log('Loaded sessions:', sessions.value)
  } catch (e) {
    console.error('Failed to load sessions:', e)
  }
}

// Create a new session
const createNewSession = async () => {
  console.log('createNewSession called')
  console.log('Module:', props.module)
  console.log('Campaign:', props.campaign)
  
  if (!props.module || !props.campaign) {
    console.error('Module or campaign not available', props.module, props.campaign)
    return
  }
  
  const campaignPath = props.campaign.directory_path || props.campaign.path
  console.log('Campaign path:', campaignPath)
  
  console.log('Creating session with:', {
    module_id: props.module.id,
    campaign_id: props.module.campaign_id,
    campaign_directory: campaignPath,
    module_number: props.module.module_number
  })
  
  try {
    const response = await invoke<{ data: any }>('create_session', {
      request: {
        module_id: props.module.id,
        campaign_id: props.module.campaign_id,
        campaign_directory: campaignPath,
        module_number: props.module.module_number
      }
    })
    
    if (response.data) {
      sessions.value.push(response.data)
      console.log('Created new session:', response.data)
      // Reload documents to show new session documents
      emit('documentsUpdated')
    }
  } catch (e) {
    console.error('Failed to create session:', e)
  }
}

// Get stage metadata
const stageMetadata = computed(() => {
  if (!props.boardConfig) return null
  const stage = props.boardConfig.stages.find((s: any) => s.key === props.stage)
  return stage || null
})

// Get next stage info
const nextStage = computed(() => {
  if (!props.boardConfig) return null
  const currentIndex = props.boardConfig.stages.findIndex((s: any) => s.key === props.stage)
  if (currentIndex === -1 || currentIndex === props.boardConfig.stages.length - 1) return null
  return props.boardConfig.stages[currentIndex + 1]
})

// Module progress percentage
const progressPercentage = computed(() => {
  if (props.module.expected_sessions === 0) return 0
  return Math.round((props.module.actual_sessions / props.module.expected_sessions) * 100)
})

// Check if module is completed
const isCompleted = computed(() => {
  return props.stage === 'completed'
})

// Get documents for current stage
const requiredDocuments = computed(() => {
  if (!stageMetadata.value) return []
  
  const noCompletionRequired = stageMetadata.value.no_completion_required_documents || []
  
  return stageMetadata.value.required_documents.map((templateId: string) => {
    const existing = props.documents.find(d => d.template_id === templateId)
    const requiresCompletion = !noCompletionRequired.includes(templateId)
    return {
      templateId,
      title: formatDocumentName(templateId),
      description: getDocumentDescription(templateId),
      exists: !!existing,
      isCompleted: existing?.completed_at != null,
      requiresCompletion,
      document: existing
    }
  })
})

const optionalDocuments = computed(() => {
  if (!stageMetadata.value) return []
  
  return stageMetadata.value.optional_documents.map((templateId: string) => {
    const existing = props.documents.find(d => d.template_id === templateId)
    return {
      templateId,
      title: formatDocumentName(templateId),
      description: getDocumentDescription(templateId),
      exists: !!existing,
      isCompleted: existing?.completed_at != null,
      document: existing
    }
  })
})

const hasDocuments = computed(() => {
  return requiredDocuments.value.length > 0 || optionalDocuments.value.length > 0
})

// Missing required documents (only those that need completion)
const missingRequiredDocuments = computed(() => {
  return requiredDocuments.value
    .filter((doc: any) => doc.requiresCompletion && (!doc.exists || !doc.isCompleted))
    .map((doc: any) => doc.templateId)
})

// Can progress to next stage
const canProgressToNext = computed(() => {
  if (!nextStage.value) return false
  if (isCompleted.value) return false
  
  // All required documents that need completion must be completed
  return requiredDocuments.value
    .filter((doc: any) => doc.requiresCompletion)
    .every((doc: any) => doc.exists && doc.isCompleted)
})

// Format document name from template ID
const formatDocumentName = (templateId: string): string => {
  return templateId
    .replace(/[-_]/g, ' ')
    .split(' ')
    .map(word => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ')
}

// Get document description based on template ID
const getDocumentDescription = (templateId: string): string => {
  const descriptions: Record<string, string> = {
    'module_overview': 'High-level overview and planning for the module',
    'quick_npc_reference': 'Quick reference for NPCs in this module',
    'session_outline': 'Outline for running sessions in this module',
    'module_mystery': 'Mystery module with investigation elements',
    'module_dungeon': 'Dungeon crawl module template',
    'module_heist': 'Heist module with planning and execution',
    'module_horror': 'Horror module with atmosphere and tension',
    'module_political': 'Political intrigue module template',
    'major_npc_tracker': 'Track major NPCs and their relationships',
    'faction_template': 'Define factions and their interactions',
    'clue_tracker': 'Track clues and revelations',
    'region_overview': 'Overview of the module\'s region',
    'document_tracker': 'Track document completion status'
  }
  
  return descriptions[templateId] || 'Module document'
}

// Format date
const formatDate = (dateString: string): string => {
  if (!dateString) return ''
  const date = new Date(dateString)
  return date.toLocaleDateString()
}

// Format session status
const formatSessionStatus = (status: string): string => {
  const statusMap: Record<string, string> = {
    'planning': 'Planning',
    'in_prep': 'In Prep',
    'ready': 'Ready',
    'running': 'Running',
    'complete': 'Complete'
  }
  return statusMap[status] || status
}

// Session management functions
const startSession = async (session: any) => {
  await transitionSessionStatus(session.id, 'in_prep')
}

const markReady = async (session: any) => {
  await transitionSessionStatus(session.id, 'ready')
}

const runSession = async (session: any) => {
  await transitionSessionStatus(session.id, 'running')
}

const completeSession = async (session: any) => {
  await transitionSessionStatus(session.id, 'complete')
}

const transitionSessionStatus = async (sessionId: number, newStatus: string) => {
  try {
    const response = await invoke<{ data: any }>('transition_session_status', {
      request: {
        session_id: sessionId,
        new_status: newStatus
      }
    })
    
    if (response.data) {
      // Update the session in our list
      const index = sessions.value.findIndex(s => s.id === sessionId)
      if (index !== -1) {
        sessions.value[index] = response.data
      }
    }
  } catch (e) {
    console.error('Failed to transition session:', e)
  }
}

// Open session document
const openSessionDocument = (session: any, docType: string) => {
  console.log('Opening', docType, 'for session', session.session_number)
  console.log('Campaign path:', props.campaign?.directory_path)
  
  if (!props.campaign?.directory_path) {
    console.error('No campaign directory path available')
    return
  }
  
  // Build the full file path for the session document
  const fileName = docType === 'outline' ? 'session-outline.md' : 'session-notes.md'
  const relativePath = `modules/module_${String(props.module.module_number).padStart(2, '0')}/session_${String(session.session_number).padStart(3, '0')}/${fileName}`
  const fullPath = `${props.campaign.directory_path}/${relativePath}`
  
  console.log('Opening session document at:', fullPath)
  
  // Create a document object that matches what the editor expects
  const sessionDocument = {
    id: `session-${session.id}-${docType}`,
    campaign_id: props.module.campaign_id,
    module_id: props.module.id,
    session_id: session.id,
    template_id: `session_${docType}`,
    document_type: `session_${docType}`,
    title: `Session ${session.session_number} ${docType === 'outline' ? 'Outline' : 'Notes'}`,
    file_path: fullPath,  // Use full path
    completed_at: null
  }
  
  emit('editDocument', sessionDocument)
}

// Get available transitions for a session status
const getAvailableTransitions = (currentStatus: string) => {
  if (!sessionBoardConfig.value) {
    // Return current status only if config not loaded
    return [{ key: currentStatus, display_name: formatSessionStatus(currentStatus) }]
  }
  
  const currentStage = sessionBoardConfig.value.stages.find((s: any) => s.key === currentStatus)
  if (!currentStage) {
    return [{ key: currentStatus, display_name: formatSessionStatus(currentStatus) }]
  }
  
  // Include current status and all valid transitions
  const transitions = [
    { key: currentStatus, display_name: currentStage.display_name }
  ]
  
  currentStage.can_transition_to.forEach((targetStatus: string) => {
    const targetStage = sessionBoardConfig.value.stages.find((s: any) => s.key === targetStatus)
    if (targetStage) {
      transitions.push({ key: targetStatus, display_name: targetStage.display_name })
    }
  })
  
  return transitions
}

// Handle session status change from dropdown
const handleSessionStatusChange = async (session: any) => {
  console.log('Session status changing to:', session.status)
  await transitionSessionStatus(session.id, session.status)
}

// Create a new document
const createDocument = (templateId: string) => {
  emit('createDocument', templateId)
}

// Edit existing document
const editDocument = (doc: any) => {
  if (doc.document) {
    emit('editDocument', doc.document)
  }
}

// Handle stage transition
const handleTransition = () => {
  if (nextStage.value) {
    emit('transitionStage', nextStage.value.key)
  }
}
</script>

<style scoped>
.stage-landing {
  padding: var(--spacing-xl);
}

.stage-header {
  margin-bottom: var(--spacing-xl);
}

.stage-header h2 {
  margin: 0 0 var(--spacing-sm) 0;
  font-size: 2rem;
  color: var(--color-text);
}

.stage-description {
  font-size: 1.125rem;
  color: var(--color-text-secondary);
  margin: 0;
}

/* Help Section */
.help-section {
  display: flex;
  gap: var(--spacing-md);
  padding: var(--spacing-lg);
  background-color: var(--color-info-bg);
  border: 1px solid var(--color-info-border);
  border-radius: var(--radius-md);
  margin-bottom: var(--spacing-xl);
}

.help-icon {
  font-size: 1.5rem;
  flex-shrink: 0;
}

.help-content p {
  margin: 0;
  color: var(--color-text);
}

/* Module Info */
.module-info {
  margin-bottom: var(--spacing-xl);
}

.info-card {
  padding: var(--spacing-lg);
  background-color: var(--color-surface-variant);
  border-radius: var(--radius-md);
}

.info-card h3 {
  margin: 0 0 var(--spacing-md) 0;
  color: var(--color-text);
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: var(--spacing-md);
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.info-item .label {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

.info-item .value {
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--color-text);
}

/* Documents Section */
.documents-section {
  margin-bottom: var(--spacing-xl);
}

.documents-section h3 {
  margin: 0 0 var(--spacing-lg) 0;
  color: var(--color-text);
}

.document-category {
  margin-bottom: var(--spacing-xl);
}

.document-category h4 {
  margin: 0 0 var(--spacing-md) 0;
  color: var(--color-text-secondary);
  font-size: 1rem;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.document-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: var(--spacing-lg);
}

.document-card {
  padding: var(--spacing-lg);
  background-color: var(--color-surface-variant);
  border: 2px solid var(--color-border);
  border-radius: var(--radius-md);
  transition: all var(--transition-base);
}

.document-card.optional {
  border-style: dashed;
  opacity: 0.9;
}

.document-card.completed {
  border-color: var(--color-success);
  background-color: var(--color-success-bg);
}

.document-card:hover {
  border-color: var(--color-primary-400);
  transform: translateY(-2px);
}

.document-status {
  margin-bottom: var(--spacing-sm);
}

.status-icon {
  font-size: 1.5rem;
}

.document-card h4 {
  margin: 0 0 var(--spacing-sm) 0;
  color: var(--color-text);
  font-size: 1rem;
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.tracking-badge {
  font-size: 0.75rem;
  color: var(--color-text-secondary);
  background-color: var(--color-surface);
  padding: 2px 6px;
  border-radius: var(--radius-xs);
  font-weight: normal;
  font-style: italic;
}

.document-description {
  margin: 0 0 var(--spacing-md) 0;
  color: var(--color-text-secondary);
  font-size: 0.875rem;
  line-height: 1.5;
}

.document-actions {
  display: flex;
  gap: var(--spacing-sm);
}

/* Session Section */
.session-section {
  margin-bottom: var(--spacing-xl);
  padding: var(--spacing-lg);
  background-color: var(--color-surface-variant);
  border-radius: var(--radius-md);
}

.session-section h3 {
  margin: 0 0 var(--spacing-lg) 0;
  color: var(--color-text);
}

.session-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-lg);
  padding: var(--spacing-md);
  background-color: var(--color-surface);
  border-radius: var(--radius-sm);
}

.session-info p {
  margin: 0;
  color: var(--color-text);
}

.sessions-list {
  margin-top: var(--spacing-lg);
}

.sessions-list h4 {
  margin: 0 0 var(--spacing-md) 0;
  color: var(--color-text-secondary);
}

.session-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  gap: var(--spacing-md);
}

.session-card {
  padding: var(--spacing-lg);
  background-color: var(--color-surface);
  border: 2px solid var(--color-border);
  border-radius: var(--radius-md);
  transition: all var(--transition-base);
}

.session-card:hover {
  border-color: var(--color-primary-400);
  transform: translateY(-2px);
}

.session-card.active {
  border-color: var(--color-primary-500);
  background-color: var(--color-primary-50);
}

.session-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-md);
}

.session-header h5 {
  margin: 0;
  color: var(--color-text);
  font-size: 1.125rem;
}

.session-status-select {
  padding: 4px 8px;
  border-radius: var(--radius-xs);
  font-size: 0.75rem;
  font-weight: 600;
  text-transform: uppercase;
  border: 1px solid transparent;
  background-color: var(--color-surface-variant);
  color: var(--color-text);
  cursor: pointer;
  transition: all var(--transition-base);
}

.session-status-select:hover {
  border-color: var(--color-primary-400);
}

.session-status-select:focus {
  outline: none;
  border-color: var(--color-primary-500);
  box-shadow: 0 0 0 2px var(--color-primary-100);
}

.session-status-select.status-next_week {
  background-color: var(--color-info-bg);
  color: var(--color-info);
}

.session-status-select.status-prep_needed {
  background-color: var(--color-warning-bg);
  color: var(--color-warning);
}

.session-status-select.status-in_prep {
  background-color: var(--color-warning-bg);
  color: var(--color-warning);
}

.session-status-select.status-ready {
  background-color: var(--color-success-bg);
  color: var(--color-success);
}

.session-status-select.status-complete {
  background-color: var(--color-surface-variant);
  color: var(--color-text-secondary);
}

.session-details {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.session-date {
  margin: 0;
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

.session-actions {
  display: flex;
  gap: var(--spacing-sm);
}

.session-documents {
  display: flex;
  gap: var(--spacing-sm);
  margin-top: var(--spacing-sm);
}

.doc-link-btn {
  padding: var(--spacing-xs) var(--spacing-sm);
  background-color: var(--color-surface-variant);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-xs);
  color: var(--color-text);
  font-size: 0.875rem;
  cursor: pointer;
  transition: all var(--transition-base);
  text-decoration: none;
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.doc-link-btn:hover {
  background-color: var(--color-primary-50);
  border-color: var(--color-primary-400);
  transform: translateY(-1px);
}

.btn-sm {
  padding: var(--spacing-xs) var(--spacing-sm);
  font-size: 0.813rem;
}

.btn-success {
  background-color: var(--color-success);
  color: white;
}

.btn-success:hover {
  background-color: var(--color-success-dark);
}

/* Transition Section */
.transition-section {
  margin-top: var(--spacing-xl);
}

.transition-card,
.completion-card,
.requirements-card {
  padding: var(--spacing-xl);
  background-color: var(--color-surface-variant);
  border-radius: var(--radius-lg);
  text-align: center;
}

.transition-card {
  border: 2px solid var(--color-primary-400);
  background-color: var(--color-surface);
  box-shadow: 0 0 0 1px var(--color-primary-200) inset;
}

.completion-card {
  border: 2px solid var(--color-success);
  background-color: var(--color-surface);
  box-shadow: 0 0 0 1px var(--color-success-light) inset;
}

.requirements-card {
  border: 2px solid var(--color-warning);
  background-color: var(--color-surface);
  box-shadow: 0 0 0 1px var(--color-warning-light) inset;
}

.transition-card h3,
.completion-card h3,
.requirements-card h3 {
  margin: 0 0 var(--spacing-md) 0;
  color: var(--color-text);
}

.transition-card p,
.completion-card p,
.requirements-card p {
  color: var(--color-text);
}

.transition-prompt {
  font-style: italic;
  color: var(--color-text);
  opacity: 0.9;
  margin: var(--spacing-md) 0;
}

.requirements-list {
  list-style: none;
  padding: 0;
  margin: var(--spacing-md) 0 0 0;
  text-align: left;
  max-width: 400px;
  margin-left: auto;
  margin-right: auto;
}

.requirements-list li {
  padding: var(--spacing-sm);
  color: var(--color-text);
}

/* Buttons */
.btn {
  padding: var(--spacing-sm) var(--spacing-md);
  border: none;
  border-radius: var(--radius-sm);
  font-size: 0.875rem;
  font-weight: 600;
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

.btn-secondary {
  background-color: var(--color-secondary);
  color: white;
}

.btn-secondary:hover {
  background-color: var(--color-secondary-dark);
}

.btn-outline {
  background-color: transparent;
  color: var(--color-primary-500);
  border: 1px solid var(--color-primary-500);
}

.btn-outline:hover {
  background-color: var(--color-primary-50);
}

.btn-large {
  padding: var(--spacing-md) var(--spacing-xl);
  font-size: 1rem;
}
</style>