<template>
  <div class="chat-view">
    <!-- Context Panel (collapsible at top) -->
    <ContextPanel :start-collapsed="true" />
    
    <!-- Main Chat Area -->
    <div class="chat-container">
      <!-- Chat History -->
      <ChatHistory
        :messages="messages"
        :is-loading="isLoading"
      />
      
      <!-- Token Usage Bar -->
      <TokenUsage
        :last-message-tokens="lastMessageTokens"
        :conversation-tokens="conversationTokens"
        :max-context="maxContextTokens"
      />
      
      <!-- Chat Input -->
      <ChatInput
        :disabled="!isReady"
        :is-loading="isLoading"
        :error="error"
        @send="handleSendMessage"
      />
    </div>
    
    <!-- Clear History Button (floating) -->
    <button
      v-if="messages.length > 0"
      @click="handleClearHistory"
      class="clear-button"
      title="Clear chat history"
    >
      Clear
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useChatStore } from '@/stores/chat'
import { useSharedContextStore } from '@/stores/sharedContext'
import ContextPanel from '../components/ContextPanel.vue'
import ChatHistory from '../components/ChatHistory.vue'
import ChatInput from '../components/ChatInput.vue'
import TokenUsage from '../components/TokenUsage.vue'

// Stores
const chatStore = useChatStore()
const contextStore = useSharedContextStore()

// State
const isReady = ref(false)

// Computed from chat store
const messages = computed(() => chatStore.messages)
const isLoading = computed(() => chatStore.isLoading)
const error = computed(() => chatStore.error)
const conversationTokens = computed(() => chatStore.conversationTokens)
const maxContextTokens = computed(() => chatStore.modelInfo?.contextLength || 262144)

// Calculate last message tokens
const lastMessageTokens = computed(() => {
  const lastMsg = chatStore.lastMessage
  return lastMsg?.tokenUsage?.total || 0
})

// Methods
const handleSendMessage = async (content: string) => {
  await chatStore.sendMessage(content)
}

const handleClearHistory = () => {
  if (confirm('Are you sure you want to clear the chat history?')) {
    chatStore.clearHistory()
  }
}

// Initialize on mount
onMounted(async () => {
  // Set window ID for this window
  (window as any).__TAURI_WINDOW_ID__ = 'chat'
  
  // Register window with context service
  await contextStore.registerWindow({
    id: 'chat',
    type: 'chat',
    title: 'Mimir Chat',
    focused: true
  })
  
  // Initialize stores
  await contextStore.loadFullContext()
  await chatStore.initialize()
  
  isReady.value = true
  
  // Clean up on unmount
  onUnmounted(() => {
    contextStore.unregisterWindow('chat')
  })
})

import { onUnmounted } from 'vue'
</script>

<style scoped>
.chat-view {
  @apply h-screen flex flex-col bg-gray-900 text-gray-100;
}

.chat-container {
  @apply flex-1 flex flex-col overflow-hidden;
}

.clear-button {
  @apply fixed bottom-24 right-4;
  @apply px-3 py-2 bg-red-600 hover:bg-red-700 text-white text-sm rounded-lg;
  @apply shadow-lg transition-colors duration-200;
  z-index: 10;
}
</style>