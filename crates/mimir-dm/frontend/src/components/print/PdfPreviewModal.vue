<template>
  <div v-if="visible" class="modal-overlay" @click="handleOverlayClick">
    <div class="modal-content pdf-modal" @click.stop>
      <div class="modal-header">
        <h2 class="modal-title">{{ title }}</h2>
        <div class="header-actions">
          <span v-if="pdfSize" class="pdf-size">{{ formatSize(pdfSize) }}</span>
          <button @click="closeModal" class="close-button">×</button>
        </div>
      </div>

      <div class="modal-body">
        <div v-if="isLoading" class="loading-state">
          <div class="spinner"></div>
          <p>Generating PDF...</p>
        </div>

        <div v-else-if="error" class="error-state">
          <p class="error-title">Failed to generate PDF</p>
          <p class="error-message">{{ error }}</p>
          <button @click="retry" class="retry-button">Try Again</button>
        </div>

        <div v-else-if="pdfUrl" class="pdf-container">
          <iframe
            :src="pdfUrl"
            class="pdf-frame"
            title="PDF Preview"
          ></iframe>
        </div>

        <div v-else class="empty-state">
          <p>No PDF to display</p>
        </div>
      </div>

      <div class="modal-footer">
        <button
          @click="handlePrint"
          class="action-button print-button"
          :disabled="!pdfUrl || isLoading"
        >
          Print
        </button>
        <button
          @click="handleSave"
          class="action-button save-button"
          :disabled="!pdfUrl || isLoading"
        >
          Save PDF
        </button>
        <button @click="closeModal" class="cancel-button">
          Close
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onUnmounted } from 'vue'
import { PrintService, type PrintResult } from '../../services/PrintService'

interface Props {
  visible: boolean
  title?: string
  defaultFileName?: string
}

const props = withDefaults(defineProps<Props>(), {
  title: 'PDF Preview',
  defaultFileName: 'document.pdf'
})

const emit = defineEmits<{
  close: []
  retry: []
}>()

const isLoading = ref(false)
const error = ref<string | null>(null)
const pdfUrl = ref<string | null>(null)
const pdfResult = ref<PrintResult | null>(null)
const pdfSize = ref<number | null>(null)

// Clean up blob URL when component unmounts or PDF changes
function cleanupUrl() {
  if (pdfUrl.value) {
    URL.revokeObjectURL(pdfUrl.value)
    pdfUrl.value = null
  }
}

onUnmounted(() => {
  cleanupUrl()
})

// Reset state when modal closes
watch(() => props.visible, (newVisible) => {
  if (!newVisible) {
    // Don't clean up URL immediately - let it persist for a moment
    // in case the modal is being reopened
    setTimeout(() => {
      if (!props.visible) {
        cleanupUrl()
        error.value = null
        pdfResult.value = null
        pdfSize.value = null
      }
    }, 100)
  }
})

// Expose methods for parent to call
function setLoading(loading: boolean) {
  isLoading.value = loading
  if (loading) {
    error.value = null
  }
}

function setError(errorMessage: string) {
  error.value = errorMessage
  isLoading.value = false
}

function setPdfResult(result: PrintResult) {
  cleanupUrl()
  pdfResult.value = result
  pdfSize.value = result.size_bytes
  pdfUrl.value = PrintService.createPdfUrl(result)
  isLoading.value = false
  error.value = null
}

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(2)} MB`
}

async function handleSave() {
  if (!pdfResult.value) return

  try {
    const savedPath = await PrintService.savePdf(pdfResult.value, props.defaultFileName)
    if (savedPath) {
      // Could show a toast notification here
      console.log('PDF saved to:', savedPath)
    }
  } catch (err) {
    console.error('Failed to save PDF:', err)
    error.value = 'Failed to save PDF. Please try again.'
  }
}

function handlePrint() {
  if (!pdfResult.value) return
  PrintService.printPdf(pdfResult.value)
}

function retry() {
  emit('retry')
}

function closeModal() {
  emit('close')
}

function handleOverlayClick() {
  closeModal()
}

// Expose methods to parent
defineExpose({
  setLoading,
  setError,
  setPdfResult
})
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: var(--color-overlay);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.pdf-modal {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-xl);
  width: 90%;
  max-width: 900px;
  height: 85vh;
  display: flex;
  flex-direction: column;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md) var(--spacing-lg);
  border-bottom: 1px solid var(--color-border);
  flex-shrink: 0;
}

.modal-title {
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--color-text);
  margin: 0;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.pdf-size {
  font-size: 0.75rem;
  color: var(--color-text-secondary);
  padding: var(--spacing-xs) var(--spacing-sm);
  background: var(--color-surface-variant);
  border-radius: var(--radius-sm);
}

.close-button {
  background: none;
  border: none;
  font-size: 1.5rem;
  color: var(--color-text-secondary);
  cursor: pointer;
  padding: 4px;
  line-height: 1;
  transition: color var(--transition-fast);
}

.close-button:hover {
  color: var(--color-text);
}

.modal-body {
  flex: 1;
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--color-gray-100);
}

.theme-dark .modal-body {
  background: var(--color-gray-900);
}

.loading-state,
.error-state,
.empty-state {
  text-align: center;
  padding: var(--spacing-xl);
}

.spinner {
  width: 40px;
  height: 40px;
  border: 3px solid var(--color-border);
  border-top-color: var(--color-primary-500);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto var(--spacing-md);
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.loading-state p {
  color: var(--color-text-secondary);
  margin: 0;
}

.error-state {
  max-width: 400px;
}

.error-title {
  font-weight: 600;
  color: var(--color-error-600);
  margin: 0 0 var(--spacing-sm);
}

.theme-dark .error-title {
  color: var(--color-error-400);
}

.error-message {
  color: var(--color-text-secondary);
  font-size: 0.875rem;
  margin: 0 0 var(--spacing-lg);
}

.retry-button {
  background: var(--color-primary-500);
  color: white;
  border: none;
  border-radius: var(--radius-md);
  padding: var(--spacing-sm) var(--spacing-lg);
  font-weight: 500;
  cursor: pointer;
  transition: background-color var(--transition-fast);
}

.retry-button:hover {
  background: var(--color-primary-600);
}

.empty-state p {
  color: var(--color-text-secondary);
  margin: 0;
}

.pdf-container {
  width: 100%;
  height: 100%;
}

.pdf-frame {
  width: 100%;
  height: 100%;
  border: none;
}

.modal-footer {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: var(--spacing-md);
  padding: var(--spacing-md) var(--spacing-lg);
  border-top: 1px solid var(--color-border);
  flex-shrink: 0;
}

.action-button {
  border: none;
  border-radius: var(--radius-md);
  padding: var(--spacing-sm) var(--spacing-lg);
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.action-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.print-button {
  background: var(--color-surface-variant);
  color: var(--color-text);
  border: 1px solid var(--color-border);
}

.print-button:hover:not(:disabled) {
  background: var(--color-gray-200);
}

.theme-dark .print-button:hover:not(:disabled) {
  background: var(--color-gray-700);
}

.save-button {
  background: var(--color-primary-500);
  color: white;
}

.save-button:hover:not(:disabled) {
  background: var(--color-primary-600);
}

.cancel-button {
  background: var(--color-surface-variant);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  padding: var(--spacing-sm) var(--spacing-lg);
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.cancel-button:hover {
  background: var(--color-gray-200);
  border-color: var(--color-border-hover);
}

.theme-dark .cancel-button:hover {
  background: var(--color-gray-700);
}
</style>
