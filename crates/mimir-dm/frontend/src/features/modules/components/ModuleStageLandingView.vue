<template>
  <div class="stage-landing">
    <!-- Module Header -->
    <div class="stage-header">
      <h2>{{ module?.name || 'Module' }}</h2>
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

    <!-- Stage-Specific Content -->
    <div class="stage-content-section">
      <!-- Planning Stage -->
      <div v-if="stage === 'planning'" class="stage-planning">
        <div class="activity-section">
          <h3>Define Your Module Concept</h3>
          <p>Develop the core conflict and structure that will drive your adventure.</p>
          
          <h4>Module Foundation</h4>
          <p>Every successful module starts with three key elements:</p>
          <ol>
            <li>
              <strong>Core Conflict:</strong> The central problem or tension
              <br><em>Example: "A cult attempts to awaken an ancient evil" or "Two guilds vie for control of a magical artifact"</em>
            </li>
            <li>
              <strong>Stakes:</strong> What happens if the PCs fail
              <br><em>Example: "The town will be destroyed" or "The balance of power shifts dangerously"</em>
            </li>
            <li>
              <strong>Hook:</strong> How the PCs get involved
              <br><em>Example: "Hired to investigate disappearances" or "Stumble upon a conspiracy"</em>
            </li>
          </ol>
        </div>
      </div>

      <!-- Development Stage -->
      <div v-else-if="stage === 'development'" class="stage-development">
        <div class="activity-section">
          <h3>Populate Your Module</h3>
          <p>Transform your module outline into playable content by creating the NPCs, encounters, and locations that will bring it to life.</p>
          
          <h4>Development Priorities</h4>
          <p>Start with your <strong>Quick NPC Reference</strong> to establish the cast of characters. This gives you a roster of personalities ready for any situation.</p>
          
          <p>If your module involves competing groups or organizations, create a <strong>Faction Template</strong> to track their goals, resources, and relationships. This helps maintain consistency as players interact with different groups.</p>
          
          <p>For modules with significant recurring characters, use the <strong>Major NPC Tracker</strong> to develop detailed profiles including motivations, secrets, and character arcs.</p>
          
          <h4>Content Creation Guidelines</h4>
          <p>Focus on creating a mix of encounters: roughly 40% combat, 30% social interaction, 20% exploration or problem-solving, and 10% unique to your module's theme. Each encounter should advance the story or reveal information.</p>
          
          <p>Design locations as more than just backdrops. Each significant location should offer opportunities for discovery, present challenges, or serve as a memorable set piece for important scenes.</p>
          
          <p>Remember the Three Clue Rule: for any critical piece of information players need, provide at least three different ways they could discover it. This prevents the adventure from stalling if players miss a single clue.</p>
        </div>
      </div>

      <!-- Ready Stage -->
      <div v-else-if="stage === 'ready'" class="stage-ready">
        <div class="activity-section">
          <h3>Final Preparations</h3>
          <p>Your module content is complete. Now it's time to prepare for actual play at the table.</p>
          
          <h4>Session Planning</h4>
          <p>Create your <strong>Session Outline</strong> to break the module into manageable chunks. Plan where natural stopping points occur, which encounters happen in which session, and how to handle pacing. A good session outline helps you maintain momentum and ensures each session ends with players eager for the next one.</p>
          
          <p>If your module involves mysteries or investigation, use the <strong>Clue Tracker</strong> to map out information flow. Document what clues exist, where they can be found, and how they connect to reveal the larger picture. This prevents you from accidentally forgetting crucial information during play.</p>
          
          <p>For modules spanning multiple locations, the <strong>Region Overview</strong> helps you maintain geographical consistency. Know travel times, local customs, and what makes each location distinct. This depth helps the world feel real when players ask unexpected questions.</p>
          
          <h4>Table Readiness</h4>
          <p>Review your module from the players' perspective. Are the hooks compelling? Do the stakes feel real? Are there multiple paths to success? This is your last chance to adjust difficulty, add alternative solutions, or strengthen weak points.</p>
          
          <p>Prepare any physical materials you'll need: maps for complex locations, handouts for important clues, reference cards for recurring NPCs. Having these ready prevents fumbling during play and maintains immersion.</p>
          
          <p>Finally, review your opening scene. The first five minutes set the tone for the entire module. Know exactly how you'll describe the initial situation, what hooks you'll emphasize, and how you'll draw players into the action.</p>
        </div>
      </div>

      <!-- Active Stage -->
      <div v-else-if="stage === 'active'" class="stage-active">
        <div class="activity-section">
          <h3>Running Your Module</h3>
          <p>Your module is now in play. Each session brings new opportunities for memorable moments and unexpected turns.</p>
          
          <h4>Session Preparation</h4>
          <p>Before each session, review your <strong>Session Outline</strong> to refresh yourself on planned encounters and pacing. Check your <strong>Session Notes</strong> from the previous game to remember where you left off, what promises you made, and what hooks the players pursued.</p>
          
          <p>The <strong>Document Tracker</strong> helps you maintain consistency across sessions. Mark which information has been revealed, which NPCs have been met, and which locations have been explored. This prevents accidental contradictions and helps you remember what the players actually know versus what only exists in your notes.</p>
          
          <h4>During Play</h4>
          <p>Stay flexible. Your preparation provides structure, but player choices create the story. When they take unexpected paths, use your documented NPCs and locations as building blocks to construct new scenarios. The module is a framework, not a script.</p>
          
          <p>Take brief notes during play: player theories (often better than your original ideas), NPC impressions they form, and promises you make. These become the seeds for future sessions and help maintain continuity.</p>
        </div>

        <div class="session-management">
          <p class="session-target">Targeting {{ module.expected_sessions }} sessions for completion.</p>

          <div class="session-controls">
            <button @click="createNewSession" class="btn btn-primary">
              + Create New Session
            </button>
          </div>
          
          <!-- Sessions Table -->
          <div v-if="sessions.length > 0" class="sessions-list">
            <table class="sessions-table">
              <thead>
                <tr>
                  <th>Session</th>
                  <th>Status</th>
                  <th>Actions</th>
                </tr>
              </thead>
              <tbody>
                <tr 
                  v-for="session in sessions" 
                  :key="session.id"
                  class="session-row"
                  :class="{ active: session.status === 'running' }"
                >
                  <td class="session-name">
                    <strong>Session #{{ session.session_number }}</strong>
                  </td>
                  <td>
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
                  </td>
                  <td class="actions-cell">
                    <button @click="openSessionDocument(session, 'outline')" class="btn btn-small btn-secondary">
                      Outline
                    </button>
                    <button @click="openSessionDocument(session, 'notes')" class="btn btn-small btn-secondary">
                      Notes
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
            
            <!-- Module Completion Button -->
            <div class="module-completion">
              <button @click="completeModule" class="btn btn-success btn-large">
                Module Completed
              </button>
              <p class="completion-note">Mark this module as complete when all sessions are finished.</p>
            </div>
          </div>

          <div v-else class="empty-sessions">
            <p>No sessions created yet. Click "Create New Session" to begin tracking your module's progress.</p>
          </div>
        </div>
      </div>

      <!-- Completed Stage -->
      <div v-else-if="stage === 'completed'" class="stage-completed">
        <div class="dashboard-section">
          <h3>Module Complete!</h3>
          <p>This module has been successfully completed. You can still review all session documents.</p>
          
          <!-- Sessions Table (Read-only for completed modules) -->
          <div v-if="sessions.length > 0" class="sessions-list">
            <table class="sessions-table">
              <thead>
                <tr>
                  <th>Session</th>
                  <th>Status</th>
                  <th>Actions</th>
                </tr>
              </thead>
              <tbody>
                <tr 
                  v-for="session in sessions" 
                  :key="session.id"
                  class="session-row"
                  :class="{ completed: session.status === 'complete' }"
                >
                  <td class="session-name">
                    <strong>Session #{{ session.session_number }}</strong>
                  </td>
                  <td>
                    <span class="status-badge" :class="`status-${session.status}`">
                      {{ formatSessionStatus(session.status) }}
                    </span>
                  </td>
                  <td class="actions-cell">
                    <button @click="openSessionDocument(session, 'outline')" class="btn btn-small btn-secondary">
                      Outline
                    </button>
                    <button @click="openSessionDocument(session, 'notes')" class="btn btn-small btn-secondary">
                      Notes
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
          
          <div v-else class="empty-sessions">
            <p>No sessions were recorded for this module.</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onActivated, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useThemeStore } from '../../../stores/theme'

// Import icon images
import lightEditIcon from '../../../assets/images/light-edit.png'
import darkEditIcon from '../../../assets/images/dark-edit.png'
import hyperEditIcon from '../../../assets/images/hyper-edit.png'

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

// Theme store for icon selection
const themeStore = useThemeStore()

// Icon mapping
const iconMap = {
  light: {
    edit: lightEditIcon
  },
  dark: {
    edit: darkEditIcon
  },
  hyper: {
    edit: hyperEditIcon
  }
}

// Get edit icon for current theme
const getEditIcon = (): string => {
  const theme = themeStore.currentTheme as 'light' | 'dark' | 'hyper'
  return iconMap[theme]?.edit || lightEditIcon
}

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
  if ((newStage === 'active' || newStage === 'completed') && newModule) {
    await loadSessionBoardConfig()
    await loadSessions()
  }
}, { immediate: true })

onMounted(async () => {
  if ((props.stage === 'active' || props.stage === 'completed') && props.module) {
    await loadSessionBoardConfig()
    await loadSessions()
  }
})

onActivated(async () => {
  if ((props.stage === 'active' || props.stage === 'completed') && props.module) {
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
    const oldStatus = sessions.value.find(s => s.id === session.id)?.status
    
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
      
      // If session transitioned to complete, we need to increment the module's actual_sessions
      if (oldStatus !== 'complete' && session.status === 'complete') {
        // Call increment_module_sessions to update the count
        try {
          const moduleResponse = await invoke<{ data: any }>('increment_module_sessions', {
            module_id: props.module.id
          })
          
          if (moduleResponse.data) {
            // Update the module prop with the new actual_sessions count
            // This will trigger reactivity and update the display
            Object.assign(props.module, moduleResponse.data)
          }
        } catch (e) {
          console.error('Failed to update module session count:', e)
        }
      }
    }
  } catch (e) {
    console.error('Failed to transition session:', e)
  }
}

// Complete the module
const completeModule = () => {
  if (!confirm('Are you sure you want to mark this module as completed? This action cannot be undone.')) {
    return
  }
  
  emit('transitionStage', 'completed')
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

/* Session Management specific styles */
.session-management {
  margin-top: var(--spacing-xl);
}

.session-target {
  font-size: 0.95rem;
  color: var(--color-text-secondary);
  margin-bottom: var(--spacing-lg);
  font-style: italic;
}

.session-controls {
  margin-bottom: var(--spacing-lg);
}

.sessions-list {
  margin-top: var(--spacing-lg);
}

/* Sessions Table */
.sessions-table {
  width: 100%;
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  border-collapse: separate;
  border-spacing: 0;
  overflow: hidden;
}

.sessions-table thead {
  background-color: var(--color-surface-variant);
}

.sessions-table th {
  padding: var(--spacing-md) var(--spacing-lg);
  text-align: left;
  font-weight: 600;
  color: var(--color-text-secondary);
  font-size: 0.875rem;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  border-bottom: 2px solid var(--color-border);
}

.sessions-table tbody tr {
  transition: background-color var(--transition-base);
}

.sessions-table tbody tr:hover {
  background-color: var(--color-surface-variant);
}

.sessions-table tbody tr:not(:last-child) {
  border-bottom: 1px solid var(--color-border);
}

.sessions-table tbody tr.active {
  background-color: var(--color-primary-50);
}

.sessions-table td {
  padding: var(--spacing-md) var(--spacing-lg);
  vertical-align: middle;
}

.session-name {
  font-size: 1rem;
}

.actions-cell {
  text-align: right;
  white-space: nowrap;
}

.actions-cell .btn {
  margin-left: var(--spacing-sm);
}

.btn-icon {
  width: 20px;
  height: 20px;
  vertical-align: middle;
  margin-right: var(--spacing-xs);
  object-fit: contain;
  display: inline-block;
}

.empty-sessions {
  padding: var(--spacing-2xl);
  text-align: center;
  color: var(--color-text-secondary);
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
}

/* Module Completion */
.module-completion {
  margin-top: var(--spacing-xl);
  padding-top: var(--spacing-xl);
  border-top: 2px solid var(--color-border);
  text-align: center;
}

.module-completion .btn-success {
  background-color: var(--color-success);
  color: white;
  font-weight: 600;
  padding: var(--spacing-md) var(--spacing-xl);
  font-size: 1rem;
}

.module-completion .btn-success:hover {
  background-color: var(--color-success-dark);
  transform: translateY(-1px);
  box-shadow: var(--shadow-md);
}

.completion-note {
  margin-top: var(--spacing-sm);
  font-size: 0.875rem;
  color: var(--color-text-secondary);
  font-style: italic;
}

/* Status select colors */
.status-select {
  padding: var(--spacing-xs) var(--spacing-sm);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  font-size: 0.875rem;
  font-weight: 600;
  cursor: pointer;
  background-color: var(--color-surface-variant);
  color: var(--color-text-secondary);
  transition: all var(--transition-base);
}

/* Session statuses using theme colors */
.status-select.status-planning {
  color: var(--color-status-planning);
}

.status-select.status-next_week {
  color: var(--color-status-next-week);
}

.status-select.status-prep_needed {
  color: var(--color-status-prep-needed);
}

.status-select.status-in_prep {
  color: var(--color-status-in-prep);
}

.status-select.status-ready {
  color: var(--color-status-ready);
}

.status-select.status-running {
  color: var(--color-status-running);
}

.status-select.status-complete {
  color: var(--color-status-complete);
}

/* Hover states for better interactivity */
.status-select:hover {
  opacity: 0.9;
  transform: translateY(-1px);
  box-shadow: var(--shadow-sm);
}

.status-select:focus {
  outline: 2px solid var(--color-primary-400);
  outline-offset: 2px;
}

/* Status Badge for completed view */
.status-badge {
  display: inline-block;
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--radius-sm);
  font-size: 0.875rem;
  font-weight: 600;
  text-transform: capitalize;
  background-color: var(--color-surface-variant);
  color: var(--color-text-secondary);
  border: 1px solid var(--color-border);
}

.status-badge.status-complete {
  background-color: var(--color-surface-variant);
  color: var(--color-success);
}

.status-badge.status-planning {
  background-color: var(--color-surface-variant);
  color: var(--color-text-secondary);
}

.status-badge.status-in_prep {
  background-color: var(--color-surface-variant);
  color: var(--color-warning);
}

.status-badge.status-ready {
  background-color: var(--color-surface-variant);
  color: var(--color-primary);
}

.status-badge.status-running {
  background-color: var(--color-surface-variant);
  color: var(--color-info);
}

/* Completed module sessions table */
.stage-completed .sessions-table tbody tr.completed {
  background-color: var(--color-surface);
  opacity: 0.9;
}

.stage-completed .dashboard-section {
  margin-top: var(--spacing-lg);
}
</style>