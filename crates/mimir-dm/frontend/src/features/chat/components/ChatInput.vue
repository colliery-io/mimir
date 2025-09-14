<template>
  <div class="chat-input-container">
    <div class="chat-input-wrapper">
      <textarea
        ref="textareaRef"
        v-model="message"
        @keydown="handleKeyDown"
        @input="adjustHeight"
        :disabled="disabled"
        :placeholder="textareaPlaceholder"
        class="form-textarea"
        :style="{ height: textareaHeight }"
      />
      <button
        @click="sendMessage"
        :disabled="!canSend"
        class="btn btn-primary chat-send-btn"
      >
        <span v-if="!isLoading && !isCancelling">Send</span>
        <span v-else-if="isCancelling">Cancelling...</span>
        <span v-else>...</span>
      </button>
    </div>
    <div v-if="error" class="form-error">
      {{ error }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, nextTick } from 'vue'

const props = defineProps<{
  disabled?: boolean
  isLoading?: boolean
  isCancelling?: boolean
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
  return message.value.trim().length > 0 && !props.disabled && !props.isLoading && !props.isCancelling
})

const textareaPlaceholder = computed(() => {
  if (props.isCancelling) {
    return 'Cancelling request... Press Escape to cancel'
  } else if (props.isLoading) {
    return 'AI is thinking... Press Escape to cancel'
  } else {
    return 'Type your message... (Ctrl+Enter to send, Escape to cancel)'
  }
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

<!-- All styling now handled by consolidated CSS classes -->