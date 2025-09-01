<template>
  <div class="chat-sidebar">
    <!-- Header -->
    <div class="sidebar-header">
      <h3 class="sidebar-title">Chat History</h3>
      <button
        @click="createNewChat"
        class="new-chat-button"
        title="New chat"
        :disabled="isCreating"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
        </svg>
      </button>
    </div>

    <!-- Loading state -->
    <div v-if="sessionsLoading" class="loading-container">
      <div class="loading-spinner"></div>
      <span class="loading-text">Loading sessions...</span>
    </div>

    <!-- Sessions list -->
    <div v-else class="sessions-list">
      <div
        v-for="session in sessions"
        :key="session.id"
        @click="switchToSession(session.id)"
        class="session-item"
        :class="{ 'active': currentSessionId === session.id }"
      >
        <div class="session-content">
          <div class="session-title">{{ session.title }}</div>
          <div class="session-preview">{{ session.preview }}</div>
          <div class="session-meta">
            <span class="session-date">{{ formatDate(session.updated_at) }}</span>
            <span class="session-count">{{ session.message_count }} messages</span>
          </div>
        </div>
        <button
          @click.stop="deleteSessionHandler(session.id)"
          class="delete-button"
          title="Delete session"
          v-if="sessions.length > 1"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
          </svg>
        </button>
      </div>
    </div>

    <!-- Empty state -->
    <div v-if="!sessionsLoading && sessions.length === 0" class="empty-state">
      <p class="empty-message">No chat sessions yet</p>
      <button @click="createNewChat" class="create-first-button">
        Start your first chat
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useChatStore } from '@/stores/chat'

const chatStore = useChatStore()

// Computed
const sessions = computed(() => chatStore.sessions)
const sessionsLoading = computed(() => chatStore.sessionsLoading)
const currentSessionId = computed(() => chatStore.currentSessionId)
const isCreating = computed(() => chatStore.isLoading)

// Methods
const switchToSession = async (sessionId: string) => {
  await chatStore.switchToSession(sessionId)
}

const createNewChat = async () => {
  await chatStore.createNewSession()
}

const deleteSessionHandler = async (sessionId: string) => {
  if (confirm('Are you sure you want to delete this chat session?')) {
    await chatStore.deleteSession(sessionId)
  }
}

const formatDate = (timestamp: number) => {
  const date = new Date(timestamp * 1000) // Convert from seconds to milliseconds
  const now = new Date()
  const diffDays = Math.floor((now.getTime() - date.getTime()) / (1000 * 60 * 60 * 24))
  
  if (diffDays === 0) {
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
  } else if (diffDays === 1) {
    return 'Yesterday'
  } else if (diffDays < 7) {
    return `${diffDays} days ago`
  } else {
    return date.toLocaleDateString([], { month: 'short', day: 'numeric' })
  }
}
</script>

<style scoped>
.chat-sidebar {
  @apply w-80 border-r flex flex-col h-full;
  background-color: var(--color-surface);
  border-color: var(--color-border);
}

.sidebar-header {
  @apply flex items-center justify-between p-4 border-b;
  border-color: var(--color-border);
}

.sidebar-title {
  @apply text-lg font-semibold;
  color: var(--color-text);
}

.new-chat-button {
  @apply p-2 rounded-lg text-white transition-colors;
  @apply disabled:opacity-50 disabled:cursor-not-allowed;
  background-color: var(--color-primary-500);
}

.new-chat-button:hover:not(:disabled) {
  background-color: var(--color-primary-600);
}

.loading-container {
  @apply flex items-center justify-center p-4;
  color: var(--color-text-secondary);
}

.loading-spinner {
  @apply w-4 h-4 border-2 rounded-full animate-spin mr-2;
  border-color: var(--color-border);
  border-top-color: var(--color-text-secondary);
}

.loading-text {
  @apply text-sm;
}

.sessions-list {
  @apply flex-1 overflow-y-auto;
}

.session-item {
  @apply flex items-start p-3 cursor-pointer border-b transition-colors duration-150;
  border-color: var(--color-border);
}

.session-item:hover {
  background-color: var(--color-surface-variant);
}

.session-item.active {
  @apply border-l-4;
  background-color: var(--color-surface-variant);
  border-left-color: var(--color-primary-500);
}

.session-content {
  @apply flex-1 min-w-0;
}

.session-title {
  @apply text-sm font-medium truncate mb-1;
  color: var(--color-text);
}

.session-preview {
  @apply text-xs line-clamp-2 mb-2;
  color: var(--color-text-secondary);
}

.session-meta {
  @apply flex items-center justify-between text-xs;
  color: var(--color-text-secondary);
}

.session-date {
  @apply truncate;
}

.session-count {
  @apply ml-2 whitespace-nowrap;
}

.delete-button {
  @apply ml-2 p-1 rounded transition-colors duration-150 opacity-0;
  color: var(--color-text-secondary);
}

.delete-button:hover {
  color: var(--color-error);
  background-color: var(--color-overlay-light);
}

.session-item:hover .delete-button {
  @apply opacity-100;
}

.empty-state {
  @apply flex flex-col items-center justify-center p-8 text-center;
  color: var(--color-text-secondary);
}

.empty-message {
  @apply text-sm mb-4;
}

.create-first-button {
  @apply px-4 py-2 text-white text-sm rounded-lg transition-colors duration-150;
  background-color: var(--color-primary-500);
}

.create-first-button:hover {
  background-color: var(--color-primary-600);
}

/* Line clamping */
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>