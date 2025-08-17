<template>
  <MainLayout>
    <div class="board-container" :class="`${entityType}-board-container`">
      <!-- Document Sidebar -->
      <slot name="sidebar" />
      
      <!-- Main Board Content -->
      <div class="board" :class="`${entityType}-board`">
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
            :style="{ zIndex: stages.length - index }"
          >
            <div class="stage-content">
              <div class="stage-name">{{ stage.name }}</div>
              <div class="stage-marker" v-if="currentStage === stage.key">●</div>
            </div>
            <div class="stage-arrow-point"></div>
          </div>
        </div>

        <!-- Main Content Area -->
        <div class="main-content">
          <slot name="content" />
        </div>
      </div>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import MainLayout from '../layout/MainLayout.vue'
import type { BoardConfig, BoardEntity, EntityType } from '../../types'

interface Props {
  entityType: EntityType
  entity: BoardEntity | null
  boardConfig: BoardConfig | null
  completedStages?: string[]
}

const props = defineProps<Props>()

// Dynamic stages from board configuration
const stages = computed(() => {
  if (!props.boardConfig) return []
  return props.boardConfig.stages.map((stage) => ({
    key: stage.key,
    name: stage.display_name.toUpperCase()
  }))
})

// Current stage based on entity status
const currentStage = computed(() => {
  if (!props.entity) return ''
  
  // Handle special case for planning -> concept mapping (campaigns)
  if (props.entityType === 'campaign' && props.entity.status === 'planning') {
    return 'concept'
  }
  
  return props.entity.status
})

// Check if a stage is completed
const isStageCompleted = (stageKey: string): boolean => {
  if (!props.completedStages) return false
  return props.completedStages.includes(stageKey)
}

// Expose computed properties for parent components
defineExpose({
  stages,
  currentStage,
  isStageCompleted
})
</script>

<style scoped>
.board-container {
  display: flex;
  height: 100%;
  gap: 0;
}

.campaign-board-container,
.module-board-container,
.session-board-container {
  /* Entity-specific styles can be added here */
}

.board {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 1.5rem;
  overflow-y: auto;
}

.campaign-board,
.module-board,
.session-board {
  /* Entity-specific board styles can be added here */
}

.stage-progress {
  display: flex;
  gap: 0;
  margin-bottom: 2rem;
  height: 42px;
  position: relative;
  padding: 0 1rem;
}

.stage-indicator {
  flex: 1;
  background-color: var(--color-surface-variant);
  border: 2px solid var(--color-border);
  padding: 0 var(--spacing-md) 0 var(--spacing-xl);
  position: relative;
  transition: all var(--transition-base);
  margin-right: -2px; /* Overlap borders */
  display: flex;
  align-items: center;
  justify-content: center;
  height: 42px;
  max-width: 200px;
  min-width: 140px;
}

.stage-indicator:first-child {
  border-radius: var(--radius-md) 0 0 var(--radius-md);
}

.stage-indicator:last-child {
  margin-right: 0;
  border-radius: 0 var(--radius-md) var(--radius-md) 0;
}

.stage-indicator.active {
  border-color: var(--color-primary-500);
}

.stage-indicator.completed {
  border-color: var(--color-success);
}

.stage-content {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-xs);
  z-index: 1;
}

.stage-name {
  font-size: 0.775rem;
  font-weight: 600;
  color: var(--color-text-secondary);
  white-space: nowrap;
}

.stage-indicator.active .stage-name,
.stage-indicator.completed .stage-name {
  color: var(--color-text);
}

.stage-marker {
  text-align: center;
  color: var(--color-primary-600);
  font-size: 0.875rem;
}

.stage-arrow-point {
  position: absolute;
  right: -20px;
  top: -2px;
  width: 0;
  height: 0;
  border-left: 20px solid var(--color-border);
  border-top: 23px solid transparent;
  border-bottom: 23px solid transparent;
  z-index: 3;
}

.stage-arrow-point::before {
  content: '';
  position: absolute;
  right: 2px;
  top: -21px;
  width: 0;
  height: 0;
  border-left: 18px solid var(--color-surface-variant);
  border-top: 21px solid transparent;
  border-bottom: 21px solid transparent;
}

.stage-indicator.active .stage-arrow-point {
  border-left-color: var(--color-primary-500);
}

.stage-indicator.active .stage-arrow-point::before {
  border-left-color: var(--color-surface-variant);
}

.stage-indicator.completed .stage-arrow-point {
  border-left-color: var(--color-success);
}

.stage-indicator.completed .stage-arrow-point::before {
  border-left-color: var(--color-surface-variant);
}

.stage-indicator:last-child .stage-arrow-point {
  display: none;
}

.main-content {
  flex: 1;
  background-color: var(--color-surface);
  border-radius: 12px;
  padding: 2rem;
  box-shadow: 0 2px 8px var(--color-shadow);
  overflow-y: auto;
}
</style>