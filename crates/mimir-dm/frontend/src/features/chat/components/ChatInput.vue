<template>
  <div class="chat-input-container">
    <div class="input-wrapper">
      <textarea
        ref="textareaRef"
        v-model="message"
        @keydown="handleKeyDown"
        @input="adjustHeight"
        :disabled="disabled"
        placeholder="Type your message... (Ctrl+Enter to send)"
        class="message-input"
        :style="{ height: textareaHeight }"
      />
      <button
        @click="sendMessage"
        :disabled="!canSend"
        class="send-button"
        :class="{ 'opacity-50 cursor-not-allowed': !canSend }"
      >
        <span v-if="!isLoading">Send</span>
        <span v-else>...</span>
      </button>
    </div>
    <div v-if="error" class="error-message">
      {{ error }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, nextTick } from 'vue'

const props = defineProps<{
  disabled?: boolean
  isLoading?: boolean
  error?: string | null
}>()

const emit = defineEmits<{
  send: [message: string]
}>()

// State
const message = ref('')
const textareaRef = ref<HTMLTextAreaElement>()
const textareaHeight = ref('60px')

// Computed
const canSend = computed(() => {
  return message.value.trim().length > 0 && !props.disabled && !props.isLoading
})

// Methods
const sendMessage = () => {
  if (canSend.value) {
    emit('send', message.value)
    message.value = ''
    nextTick(() => {
      adjustHeight()
    })
  }
}

const handleKeyDown = (event: KeyboardEvent) => {
  // Ctrl+Enter or Cmd+Enter to send
  if ((event.ctrlKey || event.metaKey) && event.key === 'Enter') {
    event.preventDefault()
    sendMessage()
  }
}

const adjustHeight = () => {
  if (textareaRef.value) {
    // Reset height to auto to get the correct scrollHeight
    textareaRef.value.style.height = 'auto'
    const scrollHeight = textareaRef.value.scrollHeight
    // Set minimum height of 60px and maximum of 200px
    const height = Math.min(Math.max(scrollHeight, 60), 200)
    textareaHeight.value = `${height}px`
  }
}

onMounted(() => {
  adjustHeight()
})
</script>

<style scoped>
.chat-input-container {
  @apply border-t p-4;
  border-color: var(--color-border);
  background-color: var(--color-surface);
}

.input-wrapper {
  @apply flex gap-2;
}

.message-input {
  @apply flex-1 rounded-lg px-3 py-2;
  @apply border focus:outline-none resize-none overflow-y-auto;
  background-color: var(--color-background);
  color: var(--color-text);
  border-color: var(--color-border);
  min-height: 60px;
  max-height: 200px;
  font-family: inherit;
  line-height: 1.5;
}

.message-input:focus {
  border-color: var(--color-primary-500);
}

.message-input:disabled {
  @apply opacity-50 cursor-not-allowed;
}

.send-button {
  @apply px-4 py-2 text-white rounded-lg transition-colors duration-200 self-end;
  background-color: var(--color-primary-500);
  min-width: 70px;
}

.send-button:hover:not(:disabled) {
  background-color: var(--color-primary-600);
}

.error-message {
  @apply mt-2 text-sm;
  color: var(--color-error);
}
</style>