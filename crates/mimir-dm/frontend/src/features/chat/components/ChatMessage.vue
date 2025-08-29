<template>
  <div
    class="chat-message"
    :class="{
      'user-message': message.role === 'user',
      'assistant-message': message.role === 'assistant',
      'system-message': message.role === 'system'
    }"
  >
    <div class="message-bubble">
      <div class="message-content" v-html="formattedContent"></div>
      <div class="message-meta">
        <span class="timestamp">{{ formatTime(message.timestamp) }}</span>
        <span v-if="message.tokenUsage" class="token-count">
          {{ message.tokenUsage.total }} tokens
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { ChatMessage } from '@/stores/chat'
import { marked } from 'marked'

const props = defineProps<{
  message: ChatMessage
}>()

// Format markdown content
const formattedContent = computed(() => {
  if (props.message.role === 'assistant') {
    // Parse markdown for assistant messages
    return marked(props.message.content, {
      breaks: true,
      gfm: true
    })
  }
  // Plain text for user messages (with line breaks preserved)
  return props.message.content.replace(/\n/g, '<br>')
})

const formatTime = (timestamp: number) => {
  const date = new Date(timestamp)
  return date.toLocaleTimeString([], { 
    hour: '2-digit', 
    minute: '2-digit' 
  })
}
</script>

<style scoped>
.chat-message {
  @apply flex mb-4;
}

.user-message {
  @apply justify-end;
}

.assistant-message {
  @apply justify-start;
}

.system-message {
  @apply justify-center;
}

.message-bubble {
  @apply max-w-[70%] rounded-lg px-4 py-2;
}

.user-message .message-bubble {
  @apply bg-blue-600 text-white;
}

.assistant-message .message-bubble {
  @apply bg-gray-700 text-gray-100;
}

.system-message .message-bubble {
  @apply bg-gray-800 text-gray-400 italic text-sm;
}

.message-content {
  @apply text-sm leading-relaxed;
}

/* Markdown content styling */
.message-content :deep(pre) {
  @apply bg-gray-900 rounded p-2 my-2 overflow-x-auto;
}

.message-content :deep(code) {
  @apply bg-gray-900 px-1 py-0.5 rounded text-xs;
}

.message-content :deep(pre code) {
  @apply bg-transparent px-0 py-0;
}

.message-content :deep(ul),
.message-content :deep(ol) {
  @apply ml-4 my-2;
}

.message-content :deep(li) {
  @apply my-1;
}

.message-content :deep(h1),
.message-content :deep(h2),
.message-content :deep(h3) {
  @apply font-semibold mt-3 mb-2;
}

.message-meta {
  @apply flex justify-between items-center mt-1 text-xs opacity-60;
}

.timestamp {
  @apply mr-2;
}

.token-count {
  @apply text-xs;
}
</style>