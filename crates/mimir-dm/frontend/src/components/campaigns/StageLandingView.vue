<template>
  <div class="stage-landing">
    <!-- Stage Header -->
    <div class="stage-header">
      <h2>{{ stageInfo.title }}</h2>
      <p class="stage-subtitle">{{ stageInfo.subtitle }}</p>
    </div>

    <!-- Progress Overview -->
    <div class="progress-section">
      <div class="progress-card">
        <div class="progress-stat">
          <span class="stat-value">{{ documentProgress.completed }}</span>
          <span class="stat-label">Documents Complete</span>
        </div>
        <div class="progress-stat">
          <span class="stat-value">{{ documentProgress.total }}</span>
          <span class="stat-label">Total Required</span>
        </div>
        <div class="progress-stat">
          <span class="stat-value">{{ documentProgress.percentage }}%</span>
          <span class="stat-label">Stage Progress</span>
        </div>
      </div>
    </div>

    <!-- Stage-Specific Content -->
    <div class="stage-content">
      <!-- Concept Stage -->
      <div v-if="stage === 'concept'" class="stage-concept">
        <div class="activity-section">
          <h3>🎯 Your Mission</h3>
          <p>Transform your campaign spark into a compelling pitch that will excite your players.</p>
          
          <div class="checklist">
            <h4>Before You Begin:</h4>
            <ul>
              <li>Have you identified your campaign's "spark" - the core idea that excites you?</li>
              <li>Can you describe your campaign in one exciting sentence?</li>
              <li>Do you know what kind of players would enjoy this campaign?</li>
            </ul>
          </div>
        </div>

        <div class="tips-section">
          <h3>💡 Tips for Success</h3>
          <div class="tip-card">
            <h4>The Big Three</h4>
            <p>Every great campaign needs:</p>
            <ol>
              <li><strong>Core Conflict:</strong> The fundamental tension driving the campaign</li>
              <li><strong>Unique Element:</strong> What makes this different from generic fantasy</li>
              <li><strong>Player Role:</strong> How the PCs fit into this world</li>
            </ol>
          </div>
          
          <div class="tip-card">
            <h4>Keep It Simple</h4>
            <p>Your pitch should be one page that sells the campaign. Save the details for later - focus on what makes it exciting!</p>
          </div>
        </div>

        <div class="action-section">
          <button class="btn-primary btn-large" @click="startDocument('campaign-pitch')">
            ✏️ Create Campaign Pitch
          </button>
        </div>
      </div>

      <!-- Session Zero Stage -->
      <div v-else-if="stage === 'session_zero'" class="stage-session-zero">
        <div class="activity-section">
          <h3>🎭 Preparing for Session Zero</h3>
          <p>Create the materials that will help your players understand and engage with your world.</p>
          
          <div class="document-grid">
            <div 
              v-for="doc in sessionZeroDocuments" 
              :key="doc.templateId"
              class="document-card"
              :class="{ completed: isDocumentComplete(doc.templateId) }"
            >
              <h4>{{ doc.title }}</h4>
              <p>{{ doc.description }}</p>
              <button 
                class="btn-small"
                @click="startDocument(doc.templateId)"
              >
                {{ isDocumentComplete(doc.templateId) ? 'Edit' : 'Create' }}
              </button>
            </div>
          </div>
        </div>

        <div class="tips-section">
          <h3>📋 Session Zero Checklist</h3>
          <ul class="checklist">
            <li>Schedule session zero with all players</li>
            <li>Prepare handouts and reference materials</li>
            <li>Plan character creation guidelines</li>
            <li>Set expectations for gameplay style</li>
            <li>Discuss safety tools and boundaries</li>
          </ul>
        </div>
      </div>

      <!-- Integration Stage -->
      <div v-else-if="stage === 'integration'" class="stage-integration">
        <div class="activity-section">
          <h3>🔗 Weaving It All Together</h3>
          <p>Connect player characters to your world and prepare for active play.</p>
          
          <div class="integration-tasks">
            <div class="task-card">
              <h4>Character Integration</h4>
              <p>Review character backstories and find connection points to your campaign themes.</p>
            </div>
            <div class="task-card">
              <h4>World Building</h4>
              <p>Expand your world based on player interests and character backgrounds.</p>
            </div>
            <div class="task-card">
              <h4>First Module</h4>
              <p>Plan your opening adventure that introduces the campaign's themes.</p>
            </div>
          </div>
        </div>
      </div>

      <!-- Active Stage -->
      <div v-else-if="stage === 'active'" class="stage-active">
        <div class="campaign-dashboard">
          <h3>🎲 Campaign is Active!</h3>
          <p>Your campaign is up and running. Use the boards to track your progress.</p>
          
          <div class="dashboard-stats">
            <div class="stat-card">
              <h4>Sessions Run</h4>
              <span class="stat-number">{{ sessionCount || 0 }}</span>
            </div>
            <div class="stat-card">
              <h4>Active Modules</h4>
              <span class="stat-number">{{ activeModules || 0 }}</span>
            </div>
            <div class="stat-card">
              <h4>Player Characters</h4>
              <span class="stat-number">{{ playerCount || 0 }}</span>
            </div>
          </div>

          <div class="quick-actions">
            <button class="btn-secondary">View Module Board</button>
            <button class="btn-secondary">View Session Board</button>
          </div>
        </div>
      </div>
    </div>

    <!-- Next Steps -->
    <div class="next-steps" v-if="nextStageAvailable">
      <h3>Ready for Next Stage?</h3>
      <p>{{ nextStagePrompt }}</p>
      <button class="btn-primary" @click="transitionToNextStage">
        Advance to {{ nextStageName }} →
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  stage: string
  documents: any[]
  campaign: any
  boardConfig: any
}>()

const emit = defineEmits<{
  createDocument: [templateId: string]
  editDocument: [document: any]
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

// Session Zero document info
const sessionZeroDocuments = [
  {
    templateId: 'starting-scenario',
    title: 'Starting Scenario',
    description: 'The opening situation that brings the party together'
  },
  {
    templateId: 'world-primer',
    title: 'World Primer',
    description: 'Essential information about your campaign setting'
  },
  {
    templateId: 'character-guidelines',
    title: 'Character Guidelines',
    description: 'Rules and suggestions for character creation'
  },
  {
    templateId: 'table-expectations',
    title: 'Table Expectations',
    description: 'Gameplay style, safety tools, and social contract'
  }
]

// Document progress is computed from actual documents

const documentProgress = computed(() => {
  const stageDocuments = props.documents.filter(doc => {
    // Filter documents relevant to current stage
    return true // TODO: Implement stage filtering
  })
  
  const completed = stageDocuments.filter(doc => doc.completed_at).length
  const total = stageDocuments.length
  const percentage = total > 0 ? Math.round((completed / total) * 100) : 0
  
  return { completed, total, percentage }
})

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
</script>

<style scoped>
.stage-landing {
  padding: var(--spacing-xl);
  max-width: 1200px;
  margin: 0 auto;
}

/* Stage Header */
.stage-header {
  text-align: center;
  margin-bottom: var(--spacing-2xl);
}

.stage-header h2 {
  margin: 0 0 var(--spacing-sm) 0;
  font-size: 2rem;
  color: var(--color-text);
}

.stage-subtitle {
  margin: 0;
  font-size: 1.125rem;
  color: var(--color-text-secondary);
}

/* Progress Section */
.progress-section {
  margin-bottom: var(--spacing-2xl);
}

.progress-card {
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--spacing-xl);
  display: flex;
  justify-content: space-around;
  align-items: center;
}

.progress-stat {
  text-align: center;
}

.stat-value {
  display: block;
  font-size: 2.5rem;
  font-weight: 700;
  color: var(--color-primary-600);
  margin-bottom: var(--spacing-xs);
}

.stat-label {
  display: block;
  font-size: 0.875rem;
  color: var(--color-text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

/* Stage Content */
.stage-content {
  margin-bottom: var(--spacing-2xl);
}

.activity-section,
.tips-section {
  margin-bottom: var(--spacing-xl);
}

.activity-section h3,
.tips-section h3 {
  margin: 0 0 var(--spacing-md) 0;
  font-size: 1.5rem;
  color: var(--color-text);
}

/* Checklist */
.checklist {
  background-color: var(--color-surface-variant);
  border-radius: var(--radius-md);
  padding: var(--spacing-lg);
  margin: var(--spacing-lg) 0;
}

.checklist h4 {
  margin: 0 0 var(--spacing-md) 0;
  color: var(--color-text);
}

.checklist ul {
  margin: 0;
  padding-left: var(--spacing-lg);
}

.checklist li {
  margin-bottom: var(--spacing-sm);
  color: var(--color-text-secondary);
}

/* Tip Cards */
.tip-card {
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  padding: var(--spacing-lg);
  margin-bottom: var(--spacing-md);
}

.tip-card h4 {
  margin: 0 0 var(--spacing-sm) 0;
  color: var(--color-primary-600);
}

.tip-card p {
  margin: 0 0 var(--spacing-sm) 0;
  color: var(--color-text-secondary);
}

.tip-card ol {
  margin: 0;
  padding-left: var(--spacing-lg);
}

.tip-card li {
  margin-bottom: var(--spacing-xs);
}

/* Action Section */
.action-section {
  text-align: center;
  margin: var(--spacing-2xl) 0;
}

.btn-large {
  padding: var(--spacing-md) var(--spacing-xl);
  font-size: 1.125rem;
}

/* Document Grid */
.document-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: var(--spacing-lg);
  margin-top: var(--spacing-lg);
}

.document-card {
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  padding: var(--spacing-lg);
  transition: all var(--transition-base);
}

.document-card:hover {
  border-color: var(--color-primary-300);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.document-card.completed {
  border-color: var(--color-success);
  background-color: var(--color-success-50);
}

.document-card h4 {
  margin: 0 0 var(--spacing-sm) 0;
  color: var(--color-text);
}

.document-card p {
  margin: 0 0 var(--spacing-md) 0;
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

/* Integration Tasks */
.integration-tasks {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: var(--spacing-lg);
  margin-top: var(--spacing-lg);
}

.task-card {
  background-color: var(--color-surface-variant);
  border-radius: var(--radius-md);
  padding: var(--spacing-lg);
}

.task-card h4 {
  margin: 0 0 var(--spacing-sm) 0;
  color: var(--color-primary-600);
}

/* Campaign Dashboard */
.campaign-dashboard {
  text-align: center;
}

.dashboard-stats {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: var(--spacing-lg);
  margin: var(--spacing-xl) 0;
}

.stat-card {
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  padding: var(--spacing-lg);
}

.stat-card h4 {
  margin: 0 0 var(--spacing-sm) 0;
  font-size: 0.875rem;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--color-text-secondary);
}

.stat-number {
  font-size: 2rem;
  font-weight: 700;
  color: var(--color-primary-600);
}

.quick-actions {
  display: flex;
  gap: var(--spacing-md);
  justify-content: center;
  margin-top: var(--spacing-xl);
}

/* Next Steps */
.next-steps {
  background-color: var(--color-primary-50);
  border: 2px solid var(--color-primary-200);
  border-radius: var(--radius-lg);
  padding: var(--spacing-xl);
  text-align: center;
}

.next-steps h3 {
  margin: 0 0 var(--spacing-sm) 0;
  color: var(--color-primary-700);
}

.next-steps p {
  margin: 0 0 var(--spacing-lg) 0;
  color: var(--color-primary-600);
}

/* Buttons */
.btn-primary {
  background-color: var(--color-primary-500);
  color: white;
  border: none;
  border-radius: var(--radius-md);
  padding: var(--spacing-sm) var(--spacing-lg);
  cursor: pointer;
  font-weight: 500;
  transition: background-color var(--transition-base);
}

.btn-primary:hover {
  background-color: var(--color-primary-600);
}

.btn-secondary {
  background-color: var(--color-surface);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  padding: var(--spacing-sm) var(--spacing-lg);
  cursor: pointer;
  font-weight: 500;
  transition: all var(--transition-base);
}

.btn-secondary:hover {
  background-color: var(--color-surface-variant);
  border-color: var(--color-primary-300);
}

.btn-small {
  padding: var(--spacing-xs) var(--spacing-md);
  font-size: 0.875rem;
}
</style>