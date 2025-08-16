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
          <p>Your campaign is up and running. Use the boards to track your progress.</p>
          
          <div class="campaign-stats-card">
            <div class="campaign-stat">
              <span class="stat-value">{{ sessionCount || 0 }}</span>
              <span class="stat-label">Sessions Run</span>
            </div>
            <div class="campaign-stat">
              <span class="stat-value">{{ activeModules || 0 }}</span>
              <span class="stat-label">Active Modules</span>
            </div>
            <div class="campaign-stat">
              <span class="stat-value">{{ playerCount || 0 }}</span>
              <span class="stat-label">Player Characters</span>
            </div>
          </div>

          <div class="quick-actions">
            <router-link :to="`/campaigns/${campaign.id}/modules`" class="btn btn-secondary">View Module Board</router-link>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
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

// Placeholder data (would come from props/store in real app)
const sessionCount = computed(() => 0)
const activeModules = computed(() => 0)
const playerCount = computed(() => 0)

// Methods
const transitionToNextStage = () => {
  if (!props.boardConfig) return
  const stageOrder = props.boardConfig.stages.map((s: any) => s.key)
  const currentIndex = stageOrder.indexOf(props.stage)
  if (currentIndex < stageOrder.length - 1) {
    emit('transitionStage', stageOrder[currentIndex + 1])
  }
}
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
</style>