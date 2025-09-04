<template>
  <div class="chat-sidebar">
    <!-- Header -->
    <div class="chat-sidebar__header">
      <h3 class="chat-sidebar__title">Chat History</h3>
      <button
        @click="createNewChat"
        class="btn btn-primary btn-icon"
        title="New chat"
        :disabled="isCreating"
      >
        <svg width="16" height="16" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
        </svg>
      </button>
    </div>

    <!-- Loading state -->
    <div v-if="sessionsLoading" class="loading-container">
      <div class="loading-spinner"></div>
      <span>Loading sessions...</span>
    </div>

    <!-- Sessions list -->
    <div v-else class="chat-sidebar__content">
      <ul class="chat-session-list">
        <li
          v-for="session in sessions"
          :key="session.id"
          @click="switchToSession(session.id)"
          class="chat-session-item"
          :class="{ 'chat-session-item--active': currentSessionId === session.id }"
        >
          <div class="session-content">
            <div class="chat-session-title">{{ session.title }}</div>
            <div class="chat-session-preview">{{ session.preview }}</div>
            <div class="chat-session-meta">
              <span class="session-date">{{ formatDate(session.updated_at) }}</span>
              <span class="session-count">{{ session.message_count }} messages</span>
            </div>
          </div>
          <button
            @click.stop="deleteSessionHandler(session.id)"
            class="delete-button"
            title="Delete session"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
            </svg>
          </button>
        </li>
      </ul>
    </div>

    <!-- Empty state -->
    <div v-if="!sessionsLoading && sessions.length === 0" class="empty-state">
      <p class="empty-message">No chat sessions yet</p>
      <button @click="createNewChat" class="btn btn-primary">
        Start your first chat
      </button>
    </div>
  </div>

  <!-- Delete Session Confirmation Modal -->
  <div v-if="showDeleteModal" class="modal-overlay" @click="cancelDelete">
    <div class="modal-content delete-modal" @click.stop>
      <div class="modal-header">
        <h2 class="modal-title">Delete Chat Session</h2>
      </div>
      <div class="modal-body">
        <p>Are you sure you want to delete this chat session?</p>
        <p class="warning-text">This action cannot be undone. All messages in this session will be permanently deleted.</p>
        
        <div v-if="deleteError" class="error-message">
          {{ deleteError }}
        </div>
      </div>
      <div class="modal-footer">
        <button @click="confirmDelete" class="delete-confirm-button">
          Delete Session
        </button>
        <button @click="cancelDelete" class="cancel-button">
          Cancel
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { useChatStore } from '@/stores/chat'

const chatStore = useChatStore()

// Reactive state
const showDeleteModal = ref(false)
const sessionToDelete = ref<string | null>(null)
const deleteError = ref<string | null>(null)

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

const deleteSessionHandler = (sessionId: string) => {
  sessionToDelete.value = sessionId
  deleteError.value = null
  showDeleteModal.value = true
}

const confirmDelete = async () => {
  if (!sessionToDelete.value) return

  deleteError.value = null
  try {
    await chatStore.deleteSession(sessionToDelete.value)
    showDeleteModal.value = false
    sessionToDelete.value = null
  } catch (error) {
    deleteError.value = 'Failed to delete session. Please try again.'
  }
}

const cancelDelete = () => {
  showDeleteModal.value = false
  sessionToDelete.value = null
  deleteError.value = null
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

<!-- All styling now handled by consolidated CSS classes in components/chat.css and base-modal.css -->