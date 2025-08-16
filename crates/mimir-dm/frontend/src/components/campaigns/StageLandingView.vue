<template>
  <div class="stage-landing">
    <!-- Stage Header -->
    <div class="stage-header">
      <h2>{{ stageInfo.title }}</h2>
      <p class="stage-subtitle"> Crystallize your campaign idea into a compelling one-page pitch that excites players.</p>
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
      <!-- Concept Stage -->
      <div v-if="stage === 'concept'" class="stage-concept">
        <div class="activity-section">
          
          
          <div class="tip-card">
            <h4>Focus on Three Elements</h4>
            <ol>
              <li>
                <strong>Core Conflict:</strong> The fundamental tension driving your campaign
                <br><em>Example: "Ancient seals are weakening, releasing primordial titans" or "Two merchant houses vie for control of magical trade routes"</em>
              </li>
              <li>
                <strong>Unique Element:</strong> What makes this campaign special
                <br><em>Example: "Set in a city built on a massive dragon turtle's back" or "Magic only works during eclipses"</em>
              </li>
              <li>
                <strong>Player Role:</strong> How the PCs will shape the story
                <br><em>Example: "Agents of a secret order maintaining the balance" or "Founders of a new frontier settlement"</em>
              </li>
            </ol>
          </div>
        </div>
      </div>

      <!-- Session Zero Stage -->
      <div v-else-if="stage === 'session_zero'" class="stage-session-zero">
        <div class="activity-section">
          <h3>Prepare Your Session Zero</h3>
          <p>Create player-facing materials that bridge your vision with their expectations.</p>
          
          <div class="tip-card">
            <h4>Essential Session Zero Documents</h4>
            <p>Prepare these documents before meeting with your players:</p>
            <ol>
              <li>
                <strong>Starting Scenario:</strong> Where they are and what brings them together
                <br><em>Example: "You've been in the Foreign Quarter for two weeks when the Merchant Guild offers a job..."</em>
              </li>
              <li>
                <strong>World Primer:</strong> Essential setting knowledge their characters would have
                <br><em>Example: "The five major clans, why the forges are failing, common rumors..."</em>
              </li>
              <li>
                <strong>Table Expectations:</strong> How we'll play together safely and enjoyably
                <br><em>Example: "PG-13 rating, collaborative storytelling, attendance expectations, safety tools..."</em>
              </li>
              <li>
                <strong>Character Guidelines:</strong> How to build characters that fit the campaign
                <br><em>Example: "You're outsiders with unique skills, levels 1-3, standard array..."</em>
              </li>
              <li>
                <strong>Character Integration:</strong> Questions to connect characters to the world
                <br><em>Example: "Why did you come to Ironhold? What skills make you valuable? Who do you trust?"</em>
              </li>
            </ol>
          </div>
        </div>
      </div>

      <!-- Integration Stage -->
      <div v-else-if="stage === 'integration'" class="stage-integration">
        <div class="activity-section">
          <h3>Transform Session Zero Into Your Campaign</h3>
          <p>Take player contributions from Session Zero and weave them into your world.</p>
          
          <h4>Post-Session Zero Tasks</h4>
          <p>Review what your players created and make it official:</p>
          <ol>
            <li>
              <strong>Campaign Bible:</strong> Compile all world information into one master reference
              <br><em>Example: "Incorporate player-created NPCs, locations they mentioned, backstory elements..."</em>
            </li>
            <li>
              <strong>Major NPC Tracker:</strong> Document all NPCs from player backstories and Session Zero
              <br><em>Example: "The merchant who cheated Tom's rogue, Marcus's old commander, Lisa's temple contacts..."</em>
            </li>
            <li>
              <strong>First Module Hooks:</strong> Create personal stakes for each character
              <br><em>Example: "Sarah's wizard notices forge-magic anomalies, Tom's contacts go silent..."</em>
            </li>
          </ol>
          <p class="integration-note">This stage transforms collaborative ideas into playable content.</p>
        </div>
      </div>

      <!-- Active Stage -->
      <div v-else-if="stage === 'active'" class="stage-active">
        <div class="campaign-dashboard">
          <h3>Campaign is Active!</h3>
          <p>Your campaign is up and running. Track your modules and sessions below.</p>

          <!-- Module Management Section -->
          <div class="modules-section">
            <div class="section-header">
              <h3>Modules</h3>
              <button @click="showCreateModal = true" class="btn btn-primary">
                New Module
              </button>
            </div>

            <div v-if="modulesLoading" class="loading-state">
              Loading modules...
            </div>

            <div v-else-if="modules.length === 0" class="empty-state">
              <p>No modules yet. Create your first module to get started!</p>
            </div>

            <table v-else class="modules-table">
              <thead>
                <tr>
                  <th>Module</th>
                  <th>Status</th>
                  <th>Sessions</th>
                  <th>Progress</th>
                  <th>Actions</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="module in modules" :key="module.id" class="module-row">
                  <td class="module-name">
                    <strong>Module #{{ module.module_number }}:</strong> {{ module.name }}
                  </td>
                  <td>
                    <span class="status-badge" :class="module.status">
                      {{ formatModuleStatus(module.status) }}
                    </span>
                  </td>
                  <td class="sessions-info">
                    {{ module.actual_sessions }} / {{ module.expected_sessions }}
                  </td>
                  <td class="progress-cell">
                    <div v-if="getModuleProgress(module) > 0" class="progress-bar">
                      <div class="progress-fill" :style="{ width: getModuleProgress(module) + '%' }"></div>
                    </div>
                    <span v-else class="no-progress">—</span>
                  </td>
                  <td class="actions-cell">
                    <router-link :to="`/modules/${module.id}/board`" class="btn btn-primary btn-small">
                      Open Board
                    </router-link>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>

          <!-- Create Module Modal -->
          <div v-if="showCreateModal" class="modal-overlay" @click.self="showCreateModal = false">
            <div class="modal-content">
              <h2>Create New Module</h2>
              <div class="form-group">
                <label for="module-name">Module Name:</label>
                <input 
                  id="module-name"
                  v-model="newModuleName" 
                  type="text" 
                  placeholder="Enter module name"
                  @keyup.enter="confirmCreateModule"
                />
              </div>
              <div class="form-group">
                <label for="module-type">Module Type:</label>
                <select id="module-type" v-model="newModuleType">
                  <option value="standard">Standard Adventure</option>
                  <option value="mystery">Mystery</option>
                  <option value="dungeon">Dungeon Crawl</option>
                  <option value="heist">Heist</option>
                  <option value="horror">Horror</option>
                  <option value="political">Political Intrigue</option>
                </select>
              </div>
              <div class="form-group">
                <label for="module-sessions">Expected Sessions:</label>
                <input 
                  id="module-sessions"
                  v-model.number="newModuleSessions" 
                  type="number" 
                  min="1"
                  placeholder="4"
                  @keyup.enter="confirmCreateModule"
                />
              </div>
              <div class="modal-actions">
                <button @click="showCreateModal = false" class="btn btn-secondary">
                  Cancel
                </button>
                <button @click="confirmCreateModule" class="btn btn-primary">
                  Create Module
                </button>
              </div>
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
import { useRouter } from 'vue-router'
import { boardConfigService } from '../../services/boardConfigService'

const props = defineProps<{
  stage: string
  documents: any[]
  campaign: any
  boardConfig: any
}>()

const emit = defineEmits<{
  transitionStage: [newStage: string]
}>()

const router = useRouter()

// Module state
const modules = ref<any[]>([])
const modulesLoading = ref(false)
const showCreateModal = ref(false)
const newModuleName = ref('')
const newModuleType = ref('standard')
const newModuleSessions = ref(4)

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

// Initialize board configuration service on mount
onMounted(async () => {
  if (props.boardConfig && !boardConfigService.getBoardConfig('campaign')) {
    // Transform and cache the board config in the service
    const config = {
      boardType: props.boardConfig.board_type || 'campaign',
      stages: props.boardConfig.stages.map((stage: any) => ({
        key: stage.key,
        displayName: stage.display_name,
        description: stage.description,
        requiredDocuments: stage.required_documents || [],
        optionalDocuments: stage.optional_documents || [],
        metadata: {
          displayName: stage.display_name,
          description: stage.description,
          completionMessage: stage.completion_message,
          transitionPrompt: stage.transition_prompt,
          helpText: stage.help_text
        }
      })),
      transitions: props.boardConfig.transitions || {}
    }
    
    // Cache it in the service
    boardConfigService.cacheBoard(config)
  }
})

// Check if can progress to next stage
const nextStageAvailable = computed(() => {
  // Check if all required documents are complete
  if (!props.boardConfig) return false
  const currentStageInfo = props.boardConfig.stages.find((s: any) => s.key === props.stage)
  if (!currentStageInfo) return false
  
  // Check if there's a next stage
  const stageOrder = props.boardConfig.stages.map((s: any) => s.key)
  const currentIndex = stageOrder.indexOf(props.stage)
  if (currentIndex >= stageOrder.length - 1) return false
  
  // Check if required documents are complete
  const requiredDocs = currentStageInfo.required_documents
  const completedDocs = props.documents.filter(doc => 
    requiredDocs.includes(doc.template_id) && doc.completed_at
  )
  
  return completedDocs.length === requiredDocs.length && requiredDocs.length > 0
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

// Methods
const transitionToNextStage = () => {
  if (!props.boardConfig) return
  const stageOrder = props.boardConfig.stages.map((s: any) => s.key)
  const currentIndex = stageOrder.indexOf(props.stage)
  if (currentIndex < stageOrder.length - 1) {
    emit('transitionStage', stageOrder[currentIndex + 1])
  }
}

// Module methods
const loadModules = async () => {
  if (!props.campaign?.id) return
  
  modulesLoading.value = true
  try {
    const response = await invoke<{ data: any[] }>('list_campaign_modules', {
      request: {
        campaign_id: props.campaign.id
      }
    })
    modules.value = response.data || []
  } catch (e) {
    console.error('Failed to load modules:', e)
  } finally {
    modulesLoading.value = false
  }
}

const formatModuleStatus = (status: string): string => {
  return status.split('_').map(word => 
    word.charAt(0).toUpperCase() + word.slice(1)
  ).join(' ')
}

const getModuleProgress = (module: any): number => {
  if (!module || module.expected_sessions === 0) return 0
  return Math.round((module.actual_sessions / module.expected_sessions) * 100)
}

const confirmCreateModule = async () => {
  if (!newModuleName.value.trim() || !props.campaign?.id) return
  
  try {
    const response = await invoke<{ data: any }>('create_module', {
      request: {
        campaign_id: props.campaign.id,
        name: newModuleName.value.trim(),
        module_type: newModuleType.value,
        expected_sessions: newModuleSessions.value
      }
    })
    
    if (response.data) {
      // Reset form and close modal first
      showCreateModal.value = false
      newModuleName.value = ''
      newModuleType.value = 'standard'
      newModuleSessions.value = 4
      
      // Navigate to the new module's board
      router.push(`/modules/${response.data.id}/board`)
    }
  } catch (e) {
    console.error('Failed to create module:', e)
  }
}

// Watch for stage changes to load modules when entering active stage
watch(() => props.stage, (newStage) => {
  if (newStage === 'active') {
    loadModules()
  }
})

// Load modules if already in active stage
onMounted(() => {
  if (props.stage === 'active') {
    loadModules()
  }
})

// Reload modules when component is reactivated (e.g., returning from module view)
onActivated(() => {
  if (props.stage === 'active') {
    loadModules()
  }
})
</script>

<style scoped>
/* Campaign-specific stage landing styles */
.stage-landing {
  max-width: 1200px;
  margin: 0 auto;
}

/* Campaign has centered stage headers */
.stage-header {
  text-align: center;
  margin-bottom: var(--spacing-2xl);
}

.stage-subtitle {
  margin: 0;
  font-size: 1.125rem;
  color: var(--color-text-secondary);
}

/* Campaign-specific styles */
.campaign-dashboard {
  text-align: center;
}

/* Campaign Stats Card */
.campaign-stats-card {
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--spacing-xl) var(--spacing-2xl);
  display: flex;
  justify-content: space-evenly;
  align-items: center;
  margin: var(--spacing-xl) 0;
}

.campaign-stat {
  text-align: center;
  min-width: 120px;
}

.campaign-stat .stat-value {
  display: block;
  font-size: 2rem;
  font-weight: 700;
  color: var(--color-primary-600);
  margin-bottom: var(--spacing-xs);
}

.campaign-stat .stat-label {
  display: block;
  font-size: 0.875rem;
  color: var(--color-text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.integration-tasks {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: var(--spacing-lg);
  margin-top: var(--spacing-lg);
}

.quick-actions {
  display: flex;
  gap: var(--spacing-md);
  justify-content: center;
  margin-top: var(--spacing-xl);
}

/* Module Section */
.modules-section {
  margin-top: var(--spacing-2xl);
  text-align: left;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-lg);
}

.section-header h3 {
  margin: 0;
}

/* Module Table Styles */
.modules-table {
  width: 100%;
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  border-collapse: separate;
  border-spacing: 0;
  overflow: hidden;
}

.modules-table thead {
  background-color: var(--color-surface-variant);
}

.modules-table th {
  padding: var(--spacing-md) var(--spacing-lg);
  text-align: left;
  font-weight: 600;
  color: var(--color-text-secondary);
  font-size: 0.875rem;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  border-bottom: 2px solid var(--color-border);
}

.modules-table tbody tr {
  transition: background-color var(--transition-base);
}

.modules-table tbody tr:hover {
  background-color: var(--color-surface-variant);
}

.modules-table tbody tr:not(:last-child) td {
  border-bottom: 1px solid var(--color-border);
}

.modules-table td {
  padding: var(--spacing-md) var(--spacing-lg);
  vertical-align: middle;
}

.module-name {
  font-size: 1rem;
}

.module-name strong {
  color: var(--color-text);
}

.sessions-info {
  font-weight: 600;
  color: var(--color-text);
}

.progress-cell {
  min-width: 150px;
}

.progress-bar {
  height: 16px;
  background-color: var(--color-surface-variant);
  border-radius: 8px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background-color: var(--color-primary-400);
  transition: width var(--transition-base);
}

.no-progress {
  color: var(--color-text-secondary);
  font-size: 1.25rem;
}

.actions-cell {
  text-align: right;
  white-space: nowrap;
}

/* Status Badge */
.status-badge {
  display: inline-block;
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--radius-sm);
  font-size: 0.875rem;
  font-weight: 600;
  text-transform: capitalize;
}

.status-badge.planning {
  background-color: var(--color-warning-100);
  color: var(--color-warning-700);
}

.status-badge.development {
  background-color: var(--color-info-100);
  color: var(--color-info-700);
}

.status-badge.ready {
  background-color: var(--color-success-100);
  color: var(--color-success-700);
}

.status-badge.active {
  background-color: var(--color-primary-100);
  color: var(--color-primary-700);
}

.status-badge.completed {
  background-color: var(--color-text-tertiary);
  color: var(--color-text);
}

.loading-state,
.empty-state {
  text-align: center;
  padding: var(--spacing-xl);
  color: var(--color-text-secondary);
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
}

/* Modal Styles */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: var(--color-overlay);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background-color: var(--color-surface);
  border-radius: var(--radius-lg);
  padding: var(--spacing-xl);
  max-width: 500px;
  width: 90%;
  box-shadow: var(--shadow-xl);
  border: 1px solid var(--color-border);
}

.modal-content h2 {
  margin: 0 0 var(--spacing-lg) 0;
  color: var(--color-text);
  font-size: 1.5rem;
}

.form-group {
  margin-bottom: var(--spacing-lg);
}

.form-group label {
  display: block;
  margin-bottom: var(--spacing-xs);
  font-weight: 600;
  color: var(--color-text);
  font-size: 0.875rem;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.form-group input,
.form-group select {
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  background-color: var(--color-background);
  color: var(--color-text);
  font-size: 1rem;
  transition: border-color var(--transition-fast);
}

.form-group input:focus,
.form-group select:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px rgba(var(--color-primary-rgb), 0.1);
}

.form-group input::placeholder {
  color: var(--color-text-tertiary);
}

.modal-actions {
  display: flex;
  gap: var(--spacing-md);
  justify-content: flex-end;
  margin-top: var(--spacing-xl);
}

.modal-actions .btn {
  min-width: 100px;
}
</style>