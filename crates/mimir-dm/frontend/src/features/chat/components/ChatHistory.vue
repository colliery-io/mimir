<template>
  <div ref="historyContainer" class="chat-history">
    <div v-if="messages.length === 0" class="empty-state">
      <p class="text-gray-400 text-center">
        Start a conversation by typing a message below
      </p>
    </div>
    <div v-else class="messages-container">
      <ChatMessage
        v-for="message in messages"
        :key="message.id"
        :message="message"
      />
      <div v-if="isLoading" class="loading-indicator">
        <div class="typing-dots">
          <span></span>
          <span></span>
          <span></span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick, watch } from 'vue'
import type { ChatMessage as ChatMessageType } from '@/stores/chat'
import ChatMessage from './ChatMessage.vue'

const props = defineProps<{
  messages: ChatMessageType[]
  isLoading?: boolean
}>()

// Refs
const historyContainer = ref<HTMLDivElement>()

// Auto-scroll to bottom when new messages arrive
watch(
  () => props.messages.length,
  () => {
    nextTick(() => {
      if (historyContainer.value) {
        historyContainer.value.scrollTop = historyContainer.value.scrollHeight
      }
    })
  }
)

// Also scroll when loading state changes
watch(
  () => props.isLoading,
  () => {
    if (props.isLoading) {
      nextTick(() => {
        if (historyContainer.value) {
          historyContainer.value.scrollTop = historyContainer.value.scrollHeight
        }
      })
    }
  }
)
</script>

<style scoped>
.chat-history {
  @apply flex-1 overflow-y-auto p-4;
  background-color: var(--color-background);
  scrollbar-width: thin;
}

.chat-history::-webkit-scrollbar {
  width: 8px;
}

.chat-history::-webkit-scrollbar-track {
  background-color: var(--color-background);
}

.chat-history::-webkit-scrollbar-thumb {
  background-color: var(--color-surface-variant);
  border-radius: 4px;
}

.chat-history::-webkit-scrollbar-thumb:hover {
  background-color: var(--color-border-hover);
}

.empty-state {
  @apply flex items-center justify-center h-full;
}

.messages-container {
  @apply min-h-full flex flex-col justify-end;
}

.loading-indicator {
  @apply flex justify-start mb-4;
}

.typing-dots {
  @apply bg-gray-700 rounded-lg px-4 py-3 inline-flex items-center gap-1;
}

.typing-dots span {
  @apply w-2 h-2 bg-gray-400 rounded-full;
  animation: typing 1.4s infinite;
}

.typing-dots span:nth-child(2) {
  animation-delay: 0.2s;
}

.typing-dots span:nth-child(3) {
  animation-delay: 0.4s;
}

@keyframes typing {
  0%, 60%, 100% {
    transform: translateY(0);
    opacity: 0.5;
  }
  30% {
    transform: translateY(-10px);
    opacity: 1;
  }
}
</style>