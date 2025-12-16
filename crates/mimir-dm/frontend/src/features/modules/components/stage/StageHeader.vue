<template>
  <div class="stage-header">
    <div class="header-content">
      <div class="header-text">
        <h2>{{ module?.name || 'Module' }}</h2>
        <p class="stage-subtitle">{{ stageInfo.subtitle }}</p>
      </div>
      <div class="header-actions">
        <button
          class="play-mode-button"
          @click="navigateToPlayMode"
          title="Enter Play Mode"
        >
          <span class="play-icon">&#9654;</span>
          Play Mode
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import type { Module } from '@/types'

interface StageInfo {
  title: string
  subtitle: string
  color?: string
  phase?: string
}

interface Props {
  module: Module | null
  stageInfo: StageInfo
}

const props = defineProps<Props>()
const router = useRouter()

function navigateToPlayMode() {
  if (props.module?.id) {
    router.push({ name: 'module-play', params: { id: props.module.id } })
  }
}
</script>

<style scoped>
.stage-header {
  margin-bottom: 1.5rem;
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 1rem;
}

.header-text {
  flex: 1;
}

.header-text h2 {
  margin: 0;
}

.stage-subtitle {
  margin: 0.25rem 0 0 0;
  color: var(--color-text-muted);
}

.header-actions {
  flex-shrink: 0;
}

.play-mode-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.6rem 1.2rem;
  background: var(--color-accent, #e67e22);
  color: white;
  border: none;
  border-radius: 0.375rem;
  font-weight: 600;
  font-size: 0.9rem;
  cursor: pointer;
  transition: all 0.2s;
}

.play-mode-button:hover {
  background: var(--color-accent-dark, #d35400);
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(230, 126, 34, 0.3);
}

.play-icon {
  font-size: 0.8rem;
}
</style>
