<template>
  <div class="token-usage-bar">
    <div class="token-stats">
      <span class="stat-item">
        <span class="label">This message:</span>
        <span class="value">{{ lastMessageTokens }} tokens</span>
      </span>
      <span class="stat-item">
        <span class="label">Conversation:</span>
        <span class="value">{{ formatTokens(conversationTokens) }} tokens</span>
      </span>
      <span class="stat-item">
        <span class="label">Context:</span>
        <span class="value" :class="contextClass">
          {{ contextPercentage.toFixed(1) }}% 
          ({{ formatTokens(conversationTokens) }}/{{ formatTokens(maxContext) }})
        </span>
      </span>
    </div>
    <div class="progress-bar">
      <div
        class="progress-fill"
        :class="contextClass"
        :style="{ width: `${Math.min(contextPercentage, 100)}%` }"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  lastMessageTokens: number
  conversationTokens: number
  maxContext: number
}>()

// Computed
const contextPercentage = computed(() => {
  if (props.maxContext === 0) return 0
  return (props.conversationTokens / props.maxContext) * 100
})

const contextClass = computed(() => {
  const pct = contextPercentage.value
  if (pct < 50) return 'text-green-400 bg-green-600'
  if (pct < 80) return 'text-yellow-400 bg-yellow-600'
  return 'text-red-400 bg-red-600'
})

// Methods
const formatTokens = (count: number) => {
  if (count >= 1000) {
    return `${(count / 1000).toFixed(1)}k`
  }
  return count.toString()
}
</script>

<style scoped>
.token-usage-bar {
  @apply bg-gray-800 border-t border-gray-700 px-4 py-2;
}

.token-stats {
  @apply flex justify-between items-center text-sm mb-2;
}

.stat-item {
  @apply flex items-center gap-2;
}

.label {
  @apply text-gray-400;
}

.value {
  @apply text-gray-200 font-mono;
}

.progress-bar {
  @apply h-1 bg-gray-700 rounded-full overflow-hidden;
}

.progress-fill {
  @apply h-full transition-all duration-300;
}
</style>