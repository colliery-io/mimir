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

    <!-- Sessions Management (for active and completed stages) -->
    <div v-if="showSessions" class="mt-8">
      <SessionTable
        :sessions="sessions"
        :readonly="stage === 'completed'"
        @create="handleCreateSession"
        @open-document="handleOpenSessionDocument"
        @transition="handleSessionTransition"
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

const emit = defineEmits<{
  'open-session-document': [document: any]
}>()

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
const showSessions = computed(() => props.stage === 'active' || props.stage === 'ready' || props.stage === 'completed')

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
  // Sessions are auto-numbered, no need for user input
  await createSession({
    status: 'planned'
  })
}

async function handleOpenSessionDocument(session: any, docType: 'outline' | 'notes') {
  // Build the file path for the session document
  const fileName = docType === 'outline' ? 'session-outline.md' : 'session-notes.md'
  const moduleNumber = (props.module as any).module_number || 1
  const relativePath = `modules/module_${String(moduleNumber).padStart(2, '0')}/session_${String(session.session_number).padStart(3, '0')}/${fileName}`
  
  // Get campaign directory path
  const campaignResponse = await invoke<{ data: any }>('get_campaign', { 
    id: props.module.campaign_id 
  })
  
  const fullPath = `${campaignResponse.data.directory_path}/${relativePath}`
  
  // Create a document object that matches what the editor expects
  const sessionDocument = {
    id: `session-${session.id}-${docType}`,
    campaign_id: props.module.campaign_id,
    module_id: props.module.id,
    session_id: session.id,
    template_id: `session_${docType === 'outline' ? 'outline' : 'notes'}`,
    document_type: `session_${docType}`,
    title: `Session ${session.session_number} ${docType === 'outline' ? 'Outline' : 'Notes'}`,
    file_path: fullPath,
    completed_at: null
  }
  
  emit('open-session-document', sessionDocument)
}

async function handleSessionTransition(sessionId: number | string, newStatus: string) {
  await transitionSession(sessionId, newStatus)
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