<template>
  <MainLayout>
    <div class="campaign-board-container">
      <!-- Document Sidebar -->
      <DocumentSidebar 
        v-if="campaign"
        :campaign-id="campaign.id" 
        @select-document="handleSelectDocument"
        @create-document="handleCreateDocument"
      />
      
      <!-- Main Board Content -->
      <div class="campaign-board">
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
          <div class="stage-name">{{ stage.name }}</div>
          <div class="stage-marker" v-if="currentStage === stage.key">●</div>
          <div class="stage-arrow-point"></div>
        </div>
      </div>

      <!-- Campaign Info -->
      <div class="campaign-info">
        <h1>Campaign: {{ campaign?.name || 'Loading...' }}</h1>
        <p>Stage: {{ campaign?.status || '...' }}</p>
      </div>
    </div>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import MainLayout from '../../components/layout/MainLayout.vue'
import DocumentSidebar from '../../components/campaigns/DocumentSidebar.vue'
import type { Campaign } from '../../types/campaign'

const props = defineProps<{
  id: string
}>()

// Local state
const campaign = ref<Campaign | null>(null)
const loading = ref(false)
const error = ref<string | null>(null)

// Stage definitions
const stages = [
  { key: 'concept', name: 'CONCEPT' },
  { key: 'session_zero', name: 'SESSION ZERO' },
  { key: 'integration', name: 'INTEGRATION' },
  { key: 'active', name: 'ACTIVE' }
]

// Map status to stage for display
const currentStage = computed(() => {
  if (!campaign.value) return 'concept'
  
  // Map planning status to concept stage
  if (campaign.value.status === 'planning') {
    return 'concept'
  }
  
  // Otherwise use status directly as the stage
  return campaign.value.status
})

// Check if a stage is completed (before the current stage)
const isStageCompleted = (stageKey: string): boolean => {
  const stageOrder = ['concept', 'session_zero', 'integration', 'active']
  const currentIndex = stageOrder.indexOf(currentStage.value)
  const checkIndex = stageOrder.indexOf(stageKey)
  return checkIndex < currentIndex
}

// Load campaign data
const loadCampaign = async () => {
  loading.value = true
  error.value = null
  
  try {
    const response = await invoke<{ data: Campaign }>('get_campaign', { 
      id: parseInt(props.id) 
    })
    campaign.value = response.data
    console.log('Loaded campaign:', campaign.value)
  } catch (e) {
    console.error('Failed to load campaign:', e)
    error.value = 'Failed to load campaign'
  } finally {
    loading.value = false
  }
}

// Handle document selection
const handleSelectDocument = (document: any) => {
  console.log('Selected document:', document)
  // TODO: Open document editor
}

// Handle create document
const handleCreateDocument = () => {
  console.log('Create new document')
  // TODO: Open document creation dialog
}

onMounted(() => {
  loadCampaign()
})
</script>

<style scoped>
.campaign-board-container {
  display: flex;
  height: 100%;
  overflow: hidden;
}

.campaign-board {
  flex: 1;
  padding: var(--spacing-lg);
  overflow-y: auto;
}

/* Stage Progress */
.stage-progress {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-xl) var(--spacing-lg);
  background-color: var(--color-surface);
  border-radius: var(--radius-lg);
  margin-bottom: var(--spacing-xl);
  gap: 0;
}

.stage-indicator {
  background-color: var(--color-surface-variant);
  border: 2px solid var(--color-border);
  padding: var(--spacing-sm) var(--spacing-xl) var(--spacing-sm) var(--spacing-lg);
  position: relative;
  transition: all var(--transition-base);
  margin-right: -2px; /* Overlap borders */
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}

/* First stage has rounded left corners */
.stage-indicator:first-child {
  border-radius: var(--radius-md) 0 0 var(--radius-md);
}

/* Last stage has different styling */
.stage-indicator:last-child {
  margin-right: 0;
}

.stage-indicator:last-child .stage-arrow-point {
  display: none;
}

/* Arrow point on the right */
.stage-arrow-point {
  position: absolute;
  right: -20px;
  top: 50%;
  transform: translateY(-50%);
  width: 0;
  height: 0;
  border-left: 20px solid var(--color-border);
  border-top: 20px solid transparent;
  border-bottom: 20px solid transparent;
  z-index: 3;
}

.stage-arrow-point::before {
  content: '';
  position: absolute;
  right: 2px;
  top: -18px;
  width: 0;
  height: 0;
  border-left: 18px solid var(--color-surface-variant);
  border-top: 18px solid transparent;
  border-bottom: 18px solid transparent;
}

/* Completed stages - only border changes */
.stage-indicator.completed {
  border-color: var(--color-success);
}

.stage-indicator.completed .stage-arrow-point {
  border-left-color: var(--color-success);
}

/* Active stage - only border changes */
.stage-indicator.active {
  border-color: var(--color-primary-500);
}

.stage-indicator.active .stage-arrow-point {
  border-left-color: var(--color-primary-500);
}

.stage-name {
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--color-text-secondary);
  z-index: 2;
}

.stage-indicator.active .stage-name,
.stage-indicator.completed .stage-name {
  color: var(--color-text);
}

.stage-marker {
  text-align: center;
  color: var(--color-primary-600);
  font-size: 1.25rem;
  z-index: 2;
}

.campaign-info {
  background-color: var(--color-surface);
  padding: var(--spacing-lg);
  border-radius: var(--radius-lg);
}

.campaign-info h1 {
  margin: 0 0 var(--spacing-sm) 0;
  color: var(--color-text);
}

.campaign-info p {
  margin: 0;
  color: var(--color-text-secondary);
}
</style>