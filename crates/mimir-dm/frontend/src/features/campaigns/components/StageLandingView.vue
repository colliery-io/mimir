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
        </div>
      </div>

      <!-- Concluding Stage -->
      <div v-else-if="stage === 'concluding'" class="stage-concluding">
        <div class="activity-section">
          <h3>Wrapping Up Your Campaign</h3>
          <p>Time to bring your epic story to a satisfying conclusion.</p>
          
          <div class="tip-card">
            <h4>Final Story Arcs</h4>
            <ul>
              <li>Resolve all major plot threads</li>
              <li>Give each character a meaningful ending</li>
              <li>Address unfinished business</li>
              <li>Provide closure for your players</li>
            </ul>
          </div>

          <!-- Module Management Section -->
          <div class="modules-section mt-4">
            <div class="section-header">
              <h3>Campaign Modules</h3>
              <button @click="showCreateModal = true" class="btn btn-primary">
                New Module
              </button>
            </div>
            <div v-if="modules.length === 0 && !modulesLoading" class="empty-state">
              <p>No modules created during this campaign.</p>
            </div>
            <div v-else-if="modulesLoading" class="loading-state">
              <p>Loading modules...</p>
            </div>
            <div v-else>
              <table class="modules-table">
                <thead>
                  <tr>
                    <th>Module</th>
                    <th>Status</th>
                    <th>Sessions</th>
                    <th>Actions</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="module in modules" :key="module.id">
                    <td>
                      <strong>Module {{ module.module_number }}: {{ module.name }}</strong>
                    </td>
                    <td>
                      <span class="badge" :class="`badge-${getModuleStatusColor(module.status)}`">
                        {{ module.status }}
                      </span>
                    </td>
                    <td>{{ module.actual_sessions || 0 }} / {{ module.expected_sessions }}</td>
                    <td>
                      <router-link :to="`/modules/${module.id}/board`" class="btn btn-primary btn-small">
                        View Module
                      </router-link>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>

      <!-- Completed Stage -->
      <div v-else-if="stage === 'completed'" class="stage-completed">
        <div class="activity-section">
          <div class="completion-card">
            <h3>🎉 Campaign Complete!</h3>
            <p>Congratulations on completing your campaign!</p>
            
            <div class="stats-summary mt-4">
              <h4>Campaign Statistics</h4>
              <div class="campaign-stats-card">
                <div class="campaign-stat">
                  <span class="stat-value">{{ modules.length || 0 }}</span>
                  <span class="stat-label">Modules</span>
                </div>
                <div class="campaign-stat">
                  <span class="stat-value">{{ getTotalSessions() }}</span>
                  <span class="stat-label">Sessions</span>
                </div>
                <div class="campaign-stat">
                  <span class="stat-value">{{ getTotalDocuments() }}</span>
                  <span class="stat-label">Documents</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Module Archive Section -->
          <div class="modules-archive-section mt-4">
            <h3>Campaign Modules</h3>
            <p class="archive-description">Review your completed adventures and their session notes.</p>
            
            <div v-if="modulesLoading" class="loading-state">
              Loading modules...
            </div>
            
            <div v-else-if="modules.length === 0" class="empty-state">
              <p>No modules were created during this campaign.</p>
            </div>
            
            <table v-else class="modules-table archive-table">
              <thead>
                <tr>
                  <th>Module</th>
                  <th>Sessions</th>
                  <th>Status</th>
                  <th>Actions</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="module in modules" :key="module.id">
                  <td class="module-name">
                    <strong>Module #{{ module.module_number }}:</strong> {{ module.name }}
                  </td>
                  <td class="sessions-info">
                    {{ module.actual_sessions }} sessions
                  </td>
                  <td>
                    <span class="status-badge completed">
                      Completed
                    </span>
                  </td>
                  <td class="actions-cell">
                    <router-link :to="`/modules/${module.id}/board`" class="btn btn-ghost btn-small">
                      📚 View Archive
                    </router-link>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
          
          <div class="info-card mt-4">
            <p>All campaign materials remain available for reference. You can review session notes, documents, and campaign materials anytime.</p>
          </div>
        </div>
      </div>

      <!-- Create Module Modal (shared between active and concluding stages) -->
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
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onActivated, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useRouter } from 'vue-router'
import { ModuleService } from '@/services/ModuleService'
import { boardConfigService } from '../../../services/boardConfigService'

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
  const requiredDocs = currentStageInfo.required_documents || []
  
  // If there are no required documents (like active stage), allow progression
  if (requiredDocs.length === 0) {
    return true
  }
  
  const completedDocs = props.documents.filter(doc => 
    requiredDocs.includes(doc.template_id) && doc.completed_at
  )
  
  return completedDocs.length === requiredDocs.length
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
  if (!props.campaign?.id) {
    console.log('No campaign ID available')
    return
  }
  
  console.log('Loading modules for campaign:', props.campaign.id)
  modulesLoading.value = true
  try {
    modules.value = await ModuleService.list(props.campaign.id)
    console.log('Loaded modules:', modules.value)
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

const getTotalSessions = (): number => {
  return modules.value.reduce((total, module) => 
    total + (module.actual_sessions || 0), 0
  )
}

const getTotalDocuments = (): number => {
  // Count campaign docs + estimated module/session docs
  return (props.documents?.length || 0) + 
         (modules.value.length * 3) + // Estimate 3 docs per module
         (getTotalSessions() * 2) // Estimate 2 docs per session
}

const getModuleStatusColor = (status: string): string => {
  const colors: Record<string, string> = {
    planning: 'info',
    development: 'warning',
    ready: 'success',
    active: 'primary',
    completed: 'neutral'
  }
  return colors[status] || 'ghost'
}

const confirmCreateModule = async () => {
  if (!newModuleName.value.trim() || !props.campaign?.id) {
    console.log('Missing module name or campaign ID')
    return
  }
  
  console.log('Creating module for campaign:', props.campaign.id)
  try {
    const newModule = await ModuleService.create({
      campaign_id: props.campaign.id,
      name: newModuleName.value.trim(),
      module_type: newModuleType.value
      // expected_sessions not supported by ModuleService yet
    })
    console.log('Created module:', newModule)
    
    if (newModule) {
      // Reset form and close modal first
      showCreateModal.value = false
      newModuleName.value = ''
      newModuleType.value = 'standard'
      newModuleSessions.value = 4
      
      // Navigate to the new module's board
      router.push(`/modules/${newModule.id}/board`)
    }
  } catch (e) {
    console.error('Failed to create module:', e)
  }
}

// Watch for stage changes to load modules when entering active, concluding, or completed stages
watch(() => props.stage, (newStage) => {
  if (['active', 'concluding', 'completed'].includes(newStage)) {
    loadModules()
  }
})

// Load modules if already in active, concluding, or completed stage
onMounted(() => {
  if (['active', 'concluding', 'completed'].includes(props.stage)) {
    loadModules()
  }
})

// Reload modules when component is reactivated (e.g., returning from module view)
onActivated(() => {
  if (['active', 'concluding', 'completed'].includes(props.stage)) {
    loadModules()
  }
})
</script>

<\!-- Component styles have been moved to centralized CSS files -->
