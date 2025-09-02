<template>
  <div class="tool-confirmation">
    <div class="confirmation-header">
      <span class="action-indicator">🔧</span>
      <h4>{{ confirmation.request.action.title }}</h4>
    </div>
    
    <p class="confirmation-description">
      {{ confirmation.request.action.description }}
    </p>
    
    <div class="changes-list">
      <div 
        v-for="(change, index) in confirmation.request.action.changes" 
        :key="index" 
        class="change-item"
      >
        <span class="change-bullet">•</span>
        <span class="change-content" :class="{ 'content-block': change.includes('\n') }">
          <pre v-if="change.includes('\n')">{{ change }}</pre>
          <span v-else>{{ change }}</span>
        </span>
      </div>
    </div>
    
    <div class="confirmation-buttons" v-if="confirmation.status === 'pending'">
      <button @click="handleApprove" class="btn-confirm" :disabled="isProcessing">
        <span>✓ Approve</span>
      </button>
      <button @click="handleReject" class="btn-reject" :disabled="isProcessing">
        <span>✗ Cancel</span>
      </button>
    </div>
    
    <div v-else class="confirmation-result">
      <span v-if="confirmation.status === 'confirmed'" class="result-confirmed">
        ✓ Action approved and executed
      </span>
      <span v-else class="result-rejected">
        ✗ Action cancelled
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import type { PendingConfirmation } from '@/stores/chat'

const props = defineProps<{
  confirmation: PendingConfirmation
}>()

const emit = defineEmits<{
  confirm: [id: string]
  reject: [id: string]
}>()

const isProcessing = ref(false)

// Removed risk level logic - all confirmations are treated equally

const handleApprove = async (event: Event) => {
  event.preventDefault()
  event.stopPropagation()
  
  if (isProcessing.value) return
  isProcessing.value = true
  try {
    emit('confirm', props.confirmation.request.id)
    // Don't reset isProcessing here - keep button disabled after click
  } catch (error) {
    console.error('Error confirming action:', error)
    isProcessing.value = false
  }
}

const handleReject = async (event: Event) => {
  event.preventDefault()
  event.stopPropagation()
  
  if (isProcessing.value) return
  isProcessing.value = true
  try {
    emit('reject', props.confirmation.request.id)
    // Don't reset isProcessing here - keep button disabled after click
  } catch (error) {
    console.error('Error rejecting action:', error)
    isProcessing.value = false
  }
}
</script>

<style scoped>
.tool-confirmation {
  background: var(--color-surface-elevated);
  border: 1px solid var(--color-border);
  border-radius: 8px;
  padding: 16px;
  margin: 8px 0;
  width: 100%;
}

.tool-confirmation {
  border-left: 3px solid var(--color-primary-500);
}

.confirmation-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 12px;
}

.confirmation-header h4 {
  margin: 0;
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--color-text-primary);
}

.action-indicator {
  font-size: 1.2rem;
  flex-shrink: 0;
}

.confirmation-description {
  color: var(--color-text-secondary);
  margin-bottom: 12px;
  line-height: 1.5;
}

.changes-list {
  background: var(--color-surface);
  border-radius: 4px;
  padding: 12px;
  margin-bottom: 16px;
  width: 100%;
  box-sizing: border-box;
}

.change-item {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  padding: 8px 0;
  color: var(--color-text-secondary);
  font-size: 0.9rem;
  border-bottom: 1px solid var(--color-border-light, rgba(255, 255, 255, 0.05));
}

.change-item:last-child {
  border-bottom: none;
}

.change-bullet {
  flex-shrink: 0;
  margin-right: 4px;
  color: var(--color-text-tertiary);
  margin-top: 2px;
}

.change-content {
  flex: 1;
  word-break: break-word;
}

.change-content pre {
  display: block !important;
  margin: 8px 0;
  padding: 16px;
  background: var(--color-surface-darker, rgba(0, 0, 0, 0.3));
  border: 1px solid var(--color-border, rgba(255, 255, 255, 0.1));
  border-radius: 6px;
  overflow-x: auto;
  white-space: pre-wrap;
  word-wrap: break-word;
  font-family: 'Menlo', 'Monaco', 'Courier New', monospace;
  font-size: 0.85rem;
  line-height: 1.6;
  max-height: 600px;
  overflow-y: auto;
  width: 100%;
  box-sizing: border-box;
  min-height: 100px;
}

.confirmation-buttons {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
}

.btn-confirm,
.btn-reject {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 10px 20px;
  border-radius: 6px;
  font-size: 0.95rem;
  font-weight: 600;
  border: none;
  cursor: pointer;
  transition: all 0.2s;
  min-width: 100px;
  user-select: none;
}

.btn-confirm {
  background: var(--color-success);
  color: white;
}

.btn-confirm:hover:not(:disabled) {
  background: var(--color-success-hover);
}

.btn-reject {
  background: var(--color-surface-elevated);
  color: var(--color-text-secondary);
  border: 1px solid var(--color-border);
}

.btn-reject:hover:not(:disabled) {
  background: var(--color-surface-hover);
}

.btn-confirm:disabled,
.btn-reject:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.confirmation-result {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 12px;
  border-radius: 4px;
  background: var(--color-surface);
}

.result-confirmed,
.result-rejected {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 0.9rem;
}

.result-confirmed {
  color: var(--color-success);
}

.result-rejected {
  color: var(--color-text-tertiary);
}
</style>