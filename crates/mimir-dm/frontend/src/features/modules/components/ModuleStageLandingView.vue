<template>
  <div class="stage-landing">
    <!-- Module Header -->
    <StageHeader 
      :module="module" 
      :stage-info="stageInfo" 
    />

    <!-- Next Steps (shown at top when ready) -->
    <StageTransitionCard
      :available="nextStageAvailable"
      :prompt="nextStagePrompt"
      :next-stage-name="nextStageName"
      @transition="transitionToNextStage"
    />

    <!-- Stage-Specific Content from Backend -->
    <div class="stage-content-section" v-if="stageContent">
      <div :class="`stage-${stage}`">
        <div class="activity-section" v-html="stageContent"></div>
      </div>
    </div>

    <!-- Sessions Management (for active stage) -->
    <div v-if="stage === 'active' && showSessions" class="mt-8">
      <SessionTable
        :sessions="sessions"
        @create="handleCreateSession"
        @edit="handleEditSession"
        @start="handleStartSession"
        @complete="handleCompleteSession"
        @delete="handleDeleteSession"
      />
    </div>

    <!-- Document Progress Indicator -->
    <div v-if="documentProgress.total > 0" class="progress-section mt-6">
      <h3>Document Progress</h3>
      <div class="progress-bar">
        <div 
          class="progress-fill" 
          :style="{ width: `${documentProgress.percentage}%` }"
        ></div>
      </div>
      <p>{{ documentProgress.completed }} of {{ documentProgress.total }} documents completed</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, toRefs } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Module, BoardConfig, Document } from '@/types'
import StageHeader from './stage/StageHeader.vue'
import StageTransitionCard from './stage/StageTransitionCard.vue'
import SessionTable from './session/SessionTable.vue'
import { useModuleStage } from '../composables/useModuleStage'
import { useSessionManagement } from '../composables/useSessionManagement'

interface Props {
  module: Module
  stage: string
  boardConfig: BoardConfig | null
  documents: Document[]
}

const props = defineProps<Props>()

// Convert props to refs for composables
const moduleRef = computed(() => props.module)
const stageRef = computed(() => props.stage)
const boardConfigRef = computed(() => props.boardConfig)
const documentsRef = computed(() => props.documents)

// Use composables for logic
const {
  stageInfo,
  documentProgress,
  nextStageAvailable,
  nextStageName,
  nextStagePrompt,
  transitionToNextStage
} = useModuleStage(moduleRef, stageRef, boardConfigRef, documentsRef)

const {
  sessions,
  loadSessions,
  createSession,
  updateSession,
  deleteSession,
  transitionSession
} = useSessionManagement(props.module?.id)

// Stage content from backend configuration
const stageContent = ref<string>('')
const showSessions = computed(() => props.stage === 'active' || props.stage === 'ready')

// Load stage-specific content from backend
async function loadStageContent() {
  if (!props.boardConfig || !props.stage) return
  
  try {
    // Get stage content from board config or fetch from backend
    const currentStageConfig = props.boardConfig.stages?.find((s: any) => s.key === props.stage)
    
    // Check for content in various possible fields
    const content = (currentStageConfig as any)?.content || 
                   (currentStageConfig as any)?.description ||
                   (currentStageConfig as any)?.help_text
    
    if (content) {
      stageContent.value = content
    } else {
      // Optionally fetch from backend if not in config
      const response = await invoke<{ content: string }>('get_stage_content', {
        stage: props.stage,
        moduleType: (props.module as any)?.module_type || 'standard'
      })
      stageContent.value = response.content
    }
  } catch (error) {
    stageContent.value = ''
  }
}

// Session handlers
async function handleCreateSession() {
  const sessionName = prompt('Enter session name:')
  if (sessionName) {
    await createSession({
      name: sessionName,
      status: 'planned'
    })
  }
}

async function handleEditSession(session: any) {
  const newName = prompt('Edit session name:', session.name)
  if (newName && newName !== session.name) {
    await updateSession(session.id, { name: newName })
  }
}

async function handleStartSession(sessionId: number | string) {
  await transitionSession(sessionId, 'active')
}

async function handleCompleteSession(sessionId: number | string) {
  await transitionSession(sessionId, 'completed')
}

async function handleDeleteSession(sessionId: number | string) {
  if (confirm('Are you sure you want to delete this session?')) {
    await deleteSession(sessionId)
  }
}

// Load data on mount
onMounted(async () => {
  await loadStageContent()
  if (showSessions.value) {
    await loadSessions()
  }
})
</script>

<style scoped>
.stage-landing {
  padding: 1.5rem;
}

.progress-section {
  background: var(--color-surface);
  padding: 1.5rem;
  border-radius: 0.5rem;
}

.progress-bar {
  width: 100%;
  height: 20px;
  background: var(--color-base-300);
  border-radius: 10px;
  overflow: hidden;
  margin: 1rem 0;
}

.progress-fill {
  height: 100%;
  background: var(--color-primary);
  transition: width 0.3s ease;
}

.stage-content-section {
  margin: 2rem 0;
}

.activity-section {
  background: var(--color-surface);
  padding: 1.5rem;
  border-radius: 0.5rem;
}
</style>