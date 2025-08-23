<template>
  <div class="stage-header">
    <h2>{{ module?.name || 'Module' }}</h2>
    <p class="stage-subtitle">{{ stageInfo.subtitle }}</p>
  </div>
</template>

<script setup lang="ts">
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

defineProps<Props>()
</script>