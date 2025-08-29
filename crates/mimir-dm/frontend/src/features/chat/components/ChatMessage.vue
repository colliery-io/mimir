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
      <!-- Thinking blocks (if any) -->
      <div v-if="thinkingBlocks.length > 0" class="thinking-blocks">
        <div
          v-for="(block, index) in thinkingBlocks"
          :key="index"
          class="thinking-block"
          :class="{ expanded: expandedThinking[index] }"
        >
          <button
            @click="toggleThinking(index)"
            class="thinking-toggle"
          >
            <span class="toggle-icon">{{ expandedThinking[index] ? '▼' : '▶' }}</span>
            <span class="thinking-label">thinking{{ block.preview }}</span>
          </button>
          <div v-if="expandedThinking[index]" class="thinking-content">
            {{ block.content }}
          </div>
        </div>
      </div>
      
      <!-- Main message content -->
      <div class="message-content" v-html="formattedMainContent"></div>
      
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
import { computed, ref } from 'vue'
import type { ChatMessage } from '@/stores/chat'
import { marked } from 'marked'

const props = defineProps<{
  message: ChatMessage
}>()

// Track which thinking blocks are expanded
const expandedThinking = ref<Record<number, boolean>>({})

// Parse thinking blocks from content
const parsedContent = computed(() => {
  if (props.message.role !== 'assistant') {
    return {
      thinkingBlocks: [],
      mainContent: props.message.content
    }
  }
  
  const content = props.message.content
  const thinkingBlocks: Array<{ content: string; preview: string }> = []
  
  // Match <thinking>, <think>, or variations with closing tags
  const thinkingRegex = /<think(?:ing)?>([\s\S]*?)<\/think(?:ing)?>/gi
  let lastIndex = 0
  let mainContent = ''
  
  let match
  while ((match = thinkingRegex.exec(content)) !== null) {
    // Add content before the thinking block
    mainContent += content.slice(lastIndex, match.index)
    
    // Extract thinking content
    const thinkingContent = match[1].trim()
    const preview = thinkingContent.length > 50 
      ? ': ' + thinkingContent.slice(0, 50) + '...'
      : ': ' + thinkingContent
    
    thinkingBlocks.push({
      content: thinkingContent,
      preview
    })
    
    lastIndex = thinkingRegex.lastIndex
  }
  
  // Add remaining content after last thinking block
  mainContent += content.slice(lastIndex)
  
  return {
    thinkingBlocks,
    mainContent: mainContent.trim()
  }
})

const thinkingBlocks = computed(() => parsedContent.value.thinkingBlocks)

// Format the main content (without thinking blocks)
const formattedMainContent = computed(() => {
  const mainContent = parsedContent.value.mainContent
  
  if (props.message.role === 'assistant' && mainContent) {
    // Parse markdown for assistant messages
    return marked(mainContent, {
      breaks: true,
      gfm: true
    })
  }
  // Plain text for user messages (with line breaks preserved)
  return mainContent.replace(/\n/g, '<br>')
})

const toggleThinking = (index: number) => {
  expandedThinking.value[index] = !expandedThinking.value[index]
}

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

/* Thinking blocks styling */
.thinking-blocks {
  @apply mb-2;
}

.thinking-block {
  @apply mb-1;
}

.thinking-toggle {
  @apply w-full text-left px-2 py-1 rounded text-xs;
  @apply bg-gray-800 hover:bg-gray-700 transition-colors;
  @apply flex items-center gap-1;
  @apply text-gray-400;
}

.assistant-message .thinking-toggle {
  @apply bg-gray-800 hover:bg-gray-600;
}

.toggle-icon {
  @apply text-xs;
  width: 12px;
  display: inline-block;
}

.thinking-label {
  @apply italic opacity-80;
}

.thinking-content {
  @apply mt-1 p-2 rounded text-xs;
  @apply bg-gray-800 text-gray-300;
  @apply whitespace-pre-wrap;
  @apply border border-gray-700;
  max-height: 200px;
  overflow-y: auto;
}

.thinking-content::-webkit-scrollbar {
  width: 6px;
}

.thinking-content::-webkit-scrollbar-thumb {
  @apply bg-gray-600 rounded;
}
</style>