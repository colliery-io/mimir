<template>
  <div class="stage-landing">
    <!-- Stage Header -->
    <div class="stage-header">
      <h2>{{ stageInfo.title }}</h2>
      <p class="stage-subtitle">{{ stageInfo.subtitle }}</p>
    </div>

    <!-- Next Steps (shown at top when ready) -->
    <div class="stage-transition-card" v-if="nextStageAvailable">
      <h3>Ready for Next Stage!</h3>
      <p>{{ nextStagePrompt }}</p>
      <button class="btn btn-primary btn-large" @click="transitionToNextStage">
        Advance to {{ nextStageName }} →
      </button>
    </div>

    <!-- Module Stats Dashboard -->
    <div class="module-stats-section">
      <div class="module-stats-card">
        <div class="module-stat">
          <span class="stat-value">{{ module.actual_sessions || 0 }}</span>
          <span class="stat-label">Sessions Run</span>
        </div>
        <div class="module-stat">
          <span class="stat-value">{{ availableModulesCount }}</span>
          <span class="stat-label">Modules Available</span>
        </div>
      </div>
    </div>

    <!-- Stage-Specific Content -->
    <div class="stage-content-section">
      <!-- Planning Stage -->
      <div v-if="stage === 'planning'" class="stage-planning">
        <div class="activity-section">
          <h3>Stage Objective</h3>
          <p>Develop the core concept and structure for your module.</p>
          
          <div class="document-grid" v-if="stageDocuments.length > 0">
            <div 
              v-for="doc in stageDocuments" 
              :key="doc.templateId"
              class="document-card"
              :class="{ completed: isDocumentComplete(doc.templateId) }"
            >
              <h4>{{ doc.title }}</h4>
              <p>{{ doc.description }}</p>
              <button 
                class="btn btn-small btn-primary"
                @click="startDocument(doc.templateId)"
              >
                {{ isDocumentComplete(doc.templateId) ? 'Edit' : 'Create' }}
              </button>
            </div>
          </div>
          
          <div class="checklist">
            <h4>Before You Begin:</h4>
            <ul>
              <li>Have you identified the module's core conflict and stakes?</li>
              <li>Do you know how this fits into your campaign's overall narrative?</li>
              <li>Have you decided on the module's tone and themes?</li>
            </ul>
          </div>
        </div>

        <div class="tips-section">
          <h3>Tips for Success</h3>
          <div class="tip-card">
            <h4>Module Design Principles</h4>
            <p>Every great module needs:</p>
            <ol>
              <li><strong>Clear Stakes:</strong> What happens if the PCs fail?</li>
              <li><strong>Player Agency:</strong> Multiple paths to success</li>
              <li><strong>Memorable NPCs:</strong> Characters that bring the world to life</li>
            </ol>
          </div>
          
          <div class="tip-card">
            <h4>Start Small</h4>
            <p>Focus on the core concept first. You can add complexity during development.</p>
          </div>
        </div>
      </div>

      <!-- Development Stage -->
      <div v-else-if="stage === 'development'" class="stage-development">
        <div class="activity-section">
          <h3>Building Your Module</h3>
          <p>Create the encounters, NPCs, and locations that bring your module to life.</p>
          
          <div class="document-grid">
            <div 
              v-for="doc in stageDocuments" 
              :key="doc.templateId"
              class="document-card"
              :class="{ completed: isDocumentComplete(doc.templateId) }"
            >
              <h4>{{ doc.title }}</h4>
              <p>{{ doc.description }}</p>
              <button 
                class="btn btn-small btn-primary"
                @click="startDocument(doc.templateId)"
              >
                {{ isDocumentComplete(doc.templateId) ? 'Edit' : 'Create' }}
              </button>
            </div>
          </div>

          <div class="task-grid">
            <div class="task-card">
              <h4>NPCs & Factions</h4>
              <p>Design memorable characters and their relationships.</p>
            </div>
            <div class="task-card">
              <h4>Encounters</h4>
              <p>Create combat, social, and exploration challenges.</p>
            </div>
            <div class="task-card">
              <h4>Locations</h4>
              <p>Map out key locations and their connections.</p>
            </div>
          </div>
        </div>
      </div>

      <!-- Ready Stage -->
      <div v-else-if="stage === 'ready'" class="stage-ready">
        <div class="activity-section">
          <h3>Final Preparations</h3>
          <p>Your module is almost ready to run. Complete final preparations.</p>
          
          <div class="document-grid" v-if="stageDocuments.length > 0">
            <div 
              v-for="doc in stageDocuments" 
              :key="doc.templateId"
              class="document-card"
              :class="{ completed: isDocumentComplete(doc.templateId) }"
            >
              <h4>{{ doc.title }}</h4>
              <p>{{ doc.description }}</p>
              <button 
                class="btn btn-small btn-primary"
                @click="startDocument(doc.templateId)"
              >
                {{ isDocumentComplete(doc.templateId) ? 'Edit' : 'Create' }}
              </button>
            </div>
          </div>
          
          <div class="checklist">
            <h4>Pre-Launch Checklist:</h4>
            <ul>
              <li>Review all NPCs and their motivations</li>
              <li>Prepare any handouts or maps</li>
              <li>Create session outlines</li>
              <li>Note potential player hooks</li>
            </ul>
          </div>
        </div>
      </div>

      <!-- Active Stage -->
      <div v-else-if="stage === 'active'" class="stage-active">
        <div class="dashboard-section">
          <h3>Module is Active!</h3>
          <p>Track your sessions and manage module progress.</p>
          
          <div class="stats-grid">
            <div class="stat-card">
              <h4>Sessions Run</h4>
              <span class="stat-number">{{ module.actual_sessions }}</span>
            </div>
            <div class="stat-card">
              <h4>Expected Sessions</h4>
              <span class="stat-number">{{ module.expected_sessions }}</span>
            </div>
            <div class="stat-card">
              <h4>Progress</h4>
              <span class="stat-number">{{ progressPercentage }}%</span>
            </div>
          </div>

          <div class="session-management">
            <div class="session-controls">
              <button @click="createNewSession" class="btn btn-primary">
                Create New Session
              </button>
            </div>
            
            <!-- List existing sessions -->
            <div v-if="sessions.length > 0" class="sessions-list">
              <h3>Module Sessions</h3>
              <div class="session-grid">
                <div 
                  v-for="session in sessions" 
                  :key="session.id" 
                  class="session-card"
                  :class="{ active: session.status === 'running' }"
                >
                  <div class="session-header">
                    <h4>Session #{{ session.session_number }}</h4>
                    <select 
                      v-model="session.status" 
                      @change="handleSessionStatusChange(session)"
                      class="status-select"
                      :class="`status-${session.status}`"
                    >
                      <option 
                        v-for="stage in getAvailableTransitions(session.status)" 
                        :key="stage.key"
                        :value="stage.key"
                      >
                        {{ stage.display_name }}
                      </option>
                    </select>
                  </div>
                  <div class="session-actions">
                    <button @click="openSessionDocument(session, 'outline')" class="btn btn-small btn-secondary">
                      📝 Outline
                    </button>
                    <button @click="openSessionDocument(session, 'notes')" class="btn btn-small btn-secondary">
                      📔 Notes
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Completed Stage -->
      <div v-else-if="stage === 'completed'" class="stage-completed">
        <div class="dashboard-section">
          <h3>🎉 Module Complete!</h3>
          <p>This module has been successfully completed.</p>
          
          <div class="stats-grid">
            <div class="stat-card">
              <h4>Total Sessions</h4>
              <span class="stat-number">{{ module.actual_sessions }}</span>
            </div>
            <div class="stat-card">
              <h4>Started</h4>
              <span class="stat-number">{{ formatDate(module.started_at) }}</span>
            </div>
            <div class="stat-card">
              <h4>Completed</h4>
              <span class="stat-number">{{ formatDate(module.completed_at) }}</span>
            </div>
          </div>
        </div>
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

// Get stage info from board configuration
const stageInfo = computed(() => {
  if (!props.boardConfig) {
    return { title: '', subtitle: '' }
  }
  const currentStageInfo = props.boardConfig.stages.find((s: any) => s.key === props.stage)
  if (!currentStageInfo) {
    return { title: '', subtitle: '' }
  }
  return {
    title: currentStageInfo.display_name,
    subtitle: currentStageInfo.description
  }
})

// Get required documents for the current stage
const stageDocuments = computed(() => {
  if (!props.boardConfig || !props.stage) {
    return []
  }
  
  const currentStageInfo = props.boardConfig.stages.find((s: any) => s.key === props.stage)
  if (!currentStageInfo) {
    return []
  }
  
  const requiredDocs = currentStageInfo.required_documents || []
  const noCompletionRequired = currentStageInfo.no_completion_required_documents || []
  
  return requiredDocs
    .filter((docId: string) => !noCompletionRequired.includes(docId))
    .map((docId: string) => ({
      templateId: docId,
      title: formatDocumentName(docId),
      description: getDocumentDescription(docId)
    }))
})

// Document progress computation
const documentProgress = computed(() => {
  if (!props.boardConfig) {
    return { completed: 0, total: 0, percentage: 0 }
  }
  
  const currentStageInfo = props.boardConfig.stages.find((s: any) => s.key === props.stage)
  if (!currentStageInfo) {
    return { completed: 0, total: 0, percentage: 0 }
  }
  
  const requiredDocs = currentStageInfo.required_documents || []
  const noCompletionRequired = currentStageInfo.no_completion_required_documents || []
  const completionRequiredDocs = requiredDocs.filter((docId: string) => 
    !noCompletionRequired.includes(docId)
  )
  
  const total = completionRequiredDocs.length
  const completed = completionRequiredDocs.filter((docId: string) => {
    const doc = props.documents.find(d => d.template_id === docId)
    return doc?.completed_at
  }).length
  
  const percentage = total > 0 ? Math.round((completed / total) * 100) : 0
  
  return { completed, total, percentage }
})

// Check if can progress to next stage
const nextStageAvailable = computed(() => {
  if (!props.boardConfig || props.stage === 'completed') return false
  
  const currentStageInfo = props.boardConfig.stages.find((s: any) => s.key === props.stage)
  if (!currentStageInfo) return false
  
  // Check if there's a next stage
  const stageOrder = props.boardConfig.stages.map((s: any) => s.key)
  const currentIndex = stageOrder.indexOf(props.stage)
  if (currentIndex >= stageOrder.length - 1) return false
  
  // Check if required documents are complete
  const requiredDocs = currentStageInfo.required_documents || []
  const noCompletionRequired = currentStageInfo.no_completion_required_documents || []
  const completionRequiredDocs = requiredDocs.filter((docId: string) => 
    !noCompletionRequired.includes(docId)
  )
  
  const completedDocs = props.documents.filter(doc => 
    completionRequiredDocs.includes(doc.template_id) && doc.completed_at
  )
  
  return completedDocs.length === completionRequiredDocs.length && completionRequiredDocs.length > 0
})

const nextStageName = computed(() => {
  if (!props.boardConfig) return ''
  const stageOrder = props.boardConfig.stages.map((s: any) => s.key)
  const currentIndex = stageOrder.indexOf(props.stage)
  if (currentIndex < stageOrder.length - 1) {
    const nextStage = props.boardConfig.stages[currentIndex + 1]
    return nextStage.display_name
  }
  return ''
})

const nextStagePrompt = computed(() => {
  if (!props.boardConfig) return ''
  const currentStageInfo = props.boardConfig.stages.find((s: any) => s.key === props.stage)
  return currentStageInfo?.transition_prompt || ''
})

// Module progress percentage
const progressPercentage = computed(() => {
  if (props.module.expected_sessions === 0) return 0
  return Math.round((props.module.actual_sessions / props.module.expected_sessions) * 100)
})

// Count of available modules (placeholder - would need to fetch from parent/store)
const availableModulesCount = computed(() => {
  // This would typically come from a parent component or store
  // For now, showing 1 if module is in ready/active state
  return ['ready', 'active'].includes(props.stage) ? 1 : 0
})

// Session management
watch([() => props.stage, () => props.module], async ([newStage, newModule]) => {
  if (newStage === 'active' && newModule) {
    await loadSessionBoardConfig()
    await loadSessions()
  }
}, { immediate: true })

onMounted(async () => {
  if (props.stage === 'active' && props.module) {
    await loadSessionBoardConfig()
    await loadSessions()
  }
})

onActivated(async () => {
  if (props.stage === 'active' && props.module) {
    await loadSessions()
  }
})

// Load session board configuration
const loadSessionBoardConfig = async () => {
  try {
    const response = await invoke<{ data: any }>('get_session_board_config')
    sessionBoardConfig.value = response.data
  } catch (e) {
    console.error('Failed to load session board config:', e)
  }
}

// Load sessions for the module
const loadSessions = async () => {
  if (!props.module) return
  
  try {
    const response = await invoke<{ data: any[] }>('list_module_sessions', {
      request: {
        module_id: props.module.id
      }
    })
    sessions.value = response.data || []
  } catch (e) {
    console.error('Failed to load sessions:', e)
  }
}

// Create a new session
const createNewSession = async () => {
  if (!props.module || !props.campaign) return
  
  const campaignPath = props.campaign.directory_path || props.campaign.path
  
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
      emit('documentsUpdated')
    }
  } catch (e) {
    console.error('Failed to create session:', e)
  }
}

// Get available transitions for a session status
const getAvailableTransitions = (currentStatus: string) => {
  if (!sessionBoardConfig.value) {
    return [{ key: currentStatus, display_name: formatSessionStatus(currentStatus) }]
  }
  
  const currentStage = sessionBoardConfig.value.stages.find((s: any) => s.key === currentStatus)
  if (!currentStage) {
    return [{ key: currentStatus, display_name: formatSessionStatus(currentStatus) }]
  }
  
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

// Handle session status change
const handleSessionStatusChange = async (session: any) => {
  try {
    const response = await invoke<{ data: any }>('transition_session_status', {
      request: {
        session_id: session.id,
        new_status: session.status
      }
    })
    
    if (response.data) {
      const index = sessions.value.findIndex(s => s.id === session.id)
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
  if (!props.campaign?.directory_path) return
  
  const fileName = docType === 'outline' ? 'session-outline.md' : 'session-notes.md'
  const relativePath = `modules/module_${String(props.module.module_number).padStart(2, '0')}/session_${String(session.session_number).padStart(3, '0')}/${fileName}`
  const fullPath = `${props.campaign.directory_path}/${relativePath}`
  
  const sessionDocument = {
    id: `session-${session.id}-${docType}`,
    campaign_id: props.module.campaign_id,
    module_id: props.module.id,
    session_id: session.id,
    template_id: `session_${docType}`,
    document_type: `session_${docType}`,
    title: `Session ${session.session_number} ${docType === 'outline' ? 'Outline' : 'Notes'}`,
    file_path: fullPath,
    completed_at: null
  }
  
  emit('editDocument', sessionDocument)
}

// Helper methods
const isDocumentComplete = (templateId: string) => {
  return props.documents.some(doc => 
    doc.template_id === templateId && doc.completed_at
  )
}

const startDocument = (templateId: string) => {
  const existing = props.documents.find(doc => doc.template_id === templateId)
  if (existing) {
    emit('editDocument', existing)
  } else {
    emit('createDocument', templateId)
  }
}

const transitionToNextStage = () => {
  if (!props.boardConfig) return
  const stageOrder = props.boardConfig.stages.map((s: any) => s.key)
  const currentIndex = stageOrder.indexOf(props.stage)
  if (currentIndex < stageOrder.length - 1) {
    emit('transitionStage', stageOrder[currentIndex + 1])
  }
}

const formatDocumentName = (templateId: string): string => {
  return templateId
    .replace(/[-_]/g, ' ')
    .split(' ')
    .map(word => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ')
}

const getDocumentDescription = (templateId: string): string => {
  const descriptions: Record<string, string> = {
    'module_overview': 'High-level overview and planning for the module',
    'quick_npc_reference': 'Quick reference for NPCs in this module',
    'session_outline': 'Outline for running sessions in this module',
    'major_npc_tracker': 'Track major NPCs and their relationships',
    'faction_template': 'Define factions and their interactions',
    'clue_tracker': 'Track clues and revelations',
    'region_overview': 'Overview of the module\'s region',
    'document_tracker': 'Track document completion status'
  }
  
  return descriptions[templateId] || 'Module document'
}

const formatDate = (dateString: string): string => {
  if (!dateString) return 'N/A'
  const date = new Date(dateString)
  return date.toLocaleDateString()
}

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
</script>

<style scoped>
/* Module-specific additions to common styles */
.stage-landing {
  max-width: 1200px;
  margin: 0 auto;
}

/* Module has centered stage headers like campaign */
.stage-header {
  text-align: center;
  margin-bottom: var(--spacing-2xl);
}

.stage-subtitle {
  margin: 0;
  font-size: 1.125rem;
  color: var(--color-text-secondary);
}

/* Module Stats Dashboard */
.module-stats-section {
  margin-bottom: var(--spacing-xl);
}

.module-stats-card {
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--spacing-xl) var(--spacing-2xl);
  display: flex;
  justify-content: space-evenly;
  align-items: center;
}

.module-stat {
  text-align: center;
  min-width: 120px;
}

.module-stat .stat-value {
  display: block;
  font-size: 2rem;
  font-weight: 700;
  color: var(--color-primary-600);
  margin-bottom: var(--spacing-xs);
}

.module-stat .stat-label {
  display: block;
  font-size: 0.875rem;
  color: var(--color-text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

/* Session Management specific styles */
.session-management {
  margin-top: var(--spacing-xl);
}

.session-controls {
  text-align: center;
  margin-bottom: var(--spacing-xl);
}

.sessions-list {
  margin-top: var(--spacing-xl);
}

.sessions-list h3 {
  margin: 0 0 var(--spacing-lg) 0;
  font-size: 1.5rem;
  color: var(--color-text);
}

.session-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-md);
}

.session-header h4 {
  margin: 0;
  font-size: 1.125rem;
}

.session-actions {
  display: flex;
  gap: var(--spacing-sm);
}

/* Status select colors */
.status-select {
  padding: var(--spacing-xs) var(--spacing-sm);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  background-color: var(--color-surface);
  font-size: 0.875rem;
  cursor: pointer;
}

.status-select.status-planning {
  background-color: var(--color-info-bg);
  color: var(--color-info);
}

.status-select.status-in_prep {
  background-color: var(--color-warning-bg);
  color: var(--color-warning);
}

.status-select.status-ready {
  background-color: var(--color-success-bg);
  color: var(--color-success);
}

.status-select.status-running {
  background-color: var(--color-primary-100);
  color: var(--color-primary-600);
}

.status-select.status-complete {
  background-color: var(--color-gray-100);
  color: var(--color-gray-600);
}
</style>