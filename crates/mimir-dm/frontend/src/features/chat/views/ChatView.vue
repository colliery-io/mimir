<template>
  <div class="chat-view" :class="currentTheme">
    <!-- Chat Sidebar -->
    <ChatSidebar />
    
    <!-- Main Content Area -->
    <div class="main-content">
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
      
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useChatStore } from '@/stores/chat'
import { useSharedContextStore } from '@/stores/sharedContext'
import { useThemeStore } from '@/stores/theme'
import ChatSidebar from '../components/ChatSidebar.vue'
import ContextPanel from '../components/ContextPanel.vue'
import ChatHistory from '../components/ChatHistory.vue'
import ChatInput from '../components/ChatInput.vue'
import TokenUsage from '../components/TokenUsage.vue'

// Stores
const chatStore = useChatStore()
const contextStore = useSharedContextStore()
const themeStore = useThemeStore()

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

// Theme class
const currentTheme = computed(() => `theme-${themeStore.currentTheme}`)

// Methods
const handleSendMessage = async (content: string) => {
  await chatStore.sendMessage(content)
}


// Initialize on mount
onMounted(async () => {
  // Set window ID for this window
  (window as any).__TAURI_WINDOW_ID__ = 'chat'
  
  // Initialize theme store first
  await themeStore.loadThemes()
  themeStore.applyTheme()
  await themeStore.initThemeSync()
  
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
    themeStore.cleanup()
    contextStore.unregisterWindow('chat')
  })
})

import { onUnmounted } from 'vue'
</script>

<style scoped>
.chat-view {
  @apply h-screen flex;
  background-color: var(--color-background);
  color: var(--color-text);
}

.main-content {
  @apply flex-1 flex flex-col overflow-hidden;
}


.chat-container {
  @apply flex-1 flex flex-col overflow-hidden;
}

</style>