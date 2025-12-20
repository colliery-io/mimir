<template>
  <div v-if="visible" class="modal-overlay" @click="handleOverlayClick">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h2 class="modal-title">Manage Reference Books</h2>
        <button @click="closeModal" class="close-button">×</button>
      </div>
      
      <div class="modal-body">
        <div v-if="isLoadingBooks" class="loading-message">
          Loading books...
        </div>
        
        <div v-else-if="books.length === 0" class="empty-state">
          <p>No books imported yet</p>
          <p class="empty-subtitle">Import book archives to start building your reference library</p>
        </div>
        
        <div v-else class="book-list">
          <div v-for="book in books" :key="book.id" class="book-item">
            <div class="book-info">
              <span class="book-name">{{ book.name }}</span>
              <span v-if="book.image_count" class="book-meta">{{ book.image_count }} images</span>
            </div>
            <button 
              @click="handleRemoveBook(book)" 
              class="remove-button"
              title="Remove book"
            >
              ×
            </button>
          </div>
        </div>
      </div>
      
      <div class="modal-footer">
        <div v-if="isImporting" class="import-progress">
          Importing {{ importProgress.current }}/{{ importProgress.total }}: {{ importProgress.currentName }}
        </div>
        <button @click="handleImportBook" class="import-button" :disabled="isImporting">
          {{ isImporting ? 'Importing...' : 'Import Books' }}
        </button>
        <button @click="closeModal" class="cancel-button" :disabled="isImporting">
          Close
        </button>
      </div>
    </div>
  </div>

  <!-- Delete Confirmation Modal -->
  <div v-if="showDeleteModal" class="modal-overlay">
    <div class="modal-content delete-modal" @click.stop>
      <div class="modal-header">
        <h2 class="modal-title">Remove Book</h2>
      </div>
      <div class="modal-body">
        <p>Are you sure you want to remove "<strong>{{ bookToDelete?.name }}</strong>" from your library?</p>
        <p class="warning-text">This will remove the book from your reference library.</p>
        
        <div v-if="deleteError" class="error-message">
          {{ deleteError }}
        </div>
      </div>
      <div class="modal-footer">
        <button @click="confirmDelete" class="delete-confirm-button">
          Remove Book
        </button>
        <button @click="cancelDelete" class="cancel-button">
          Cancel
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import type { BookInfo } from '../types/book'

interface Props {
  visible: boolean
}

interface Emits {
  (e: 'close'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const books = ref<BookInfo[]>([])
const isLoadingBooks = ref(false)
const isImporting = ref(false)
const importProgress = ref({ current: 0, total: 0, currentName: '' })
const showDeleteModal = ref(false)
const bookToDelete = ref<BookInfo | null>(null)
const deleteError = ref<string | null>(null)

// Load books when modal becomes visible
watch(() => props.visible, (newVisible) => {
  if (newVisible) {
    loadBooks()
  }
})

async function loadBooks() {
  try {
    isLoadingBooks.value = true
    const response = await invoke<{ success: boolean; data: BookInfo[]; message?: string }>('list_library_books')
    
    if (response.success && response.data) {
      books.value = response.data
    } else {
      books.value = []
    }
  } catch (error) {
    console.error('Failed to load books:', error)
    books.value = []
  } finally {
    isLoadingBooks.value = false
  }
}

async function handleImportBook() {
  try {
    const selected = await open({
      multiple: true,
      filters: [{
        name: 'Book Archive',
        extensions: ['tar.gz', 'gz']
      }],
      title: 'Select book archives to add to your library'
    })

    if (selected) {
      // Normalize to array
      const filePaths = Array.isArray(selected) ? selected : [selected]

      if (filePaths.length === 0) return

      isImporting.value = true
      importProgress.value = { current: 0, total: filePaths.length, currentName: '' }

      const results: { success: boolean; name: string; error?: string }[] = []

      for (let i = 0; i < filePaths.length; i++) {
        const filePath = filePaths[i]
        const fileName = filePath.split('/').pop() || filePath
        importProgress.value = { current: i + 1, total: filePaths.length, currentName: fileName }

        try {
          const response = await invoke<{ success: boolean; data?: BookInfo; message?: string }>('upload_book_archive', {
            archivePath: filePath
          })

          results.push({
            success: response.success,
            name: fileName,
            error: response.message
          })
        } catch (err) {
          results.push({
            success: false,
            name: fileName,
            error: 'Import failed'
          })
        }
      }

      isImporting.value = false

      // Show results summary
      const succeeded = results.filter(r => r.success).length
      const failed = results.filter(r => !r.success)

      if (failed.length === 0) {
        alert(`Successfully imported ${succeeded} book${succeeded !== 1 ? 's' : ''}!`)
      } else if (succeeded === 0) {
        alert(`Failed to import ${failed.length} book${failed.length !== 1 ? 's' : ''}:\n${failed.map(f => `• ${f.name}: ${f.error}`).join('\n')}`)
      } else {
        alert(`Imported ${succeeded} book${succeeded !== 1 ? 's' : ''}.\n\nFailed to import ${failed.length}:\n${failed.map(f => `• ${f.name}: ${f.error}`).join('\n')}`)
      }

      // Reload the book list
      await loadBooks()
    }
  } catch (error) {
    console.error('Failed to import books:', error)
    isImporting.value = false
  }
}

function handleRemoveBook(book: BookInfo) {
  bookToDelete.value = book
  deleteError.value = null
  showDeleteModal.value = true
}

async function confirmDelete() {
  if (!bookToDelete.value) return

  deleteError.value = null
  try {
    const response = await invoke<{ success: boolean; message?: string }>('remove_book_from_library', {
      bookId: bookToDelete.value.id
    })
    
    if (response.success) {
      showDeleteModal.value = false
      bookToDelete.value = null
      // Reload the book list
      await loadBooks()
    } else {
      deleteError.value = response.message || 'Failed to remove book'
    }
  } catch (error) {
    deleteError.value = 'Failed to remove book. Please try again.'
  }
}

function cancelDelete() {
  showDeleteModal.value = false
  bookToDelete.value = null
  deleteError.value = null
}

function closeModal() {
  emit('close')
}

function handleOverlayClick() {
  closeModal()
}
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

.modal-content {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-lg);
  width: 90%;
  max-width: 600px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-lg);
  border-bottom: 1px solid var(--color-border);
}

.modal-title {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--color-text);
  margin: 0;
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
  overflow-y: auto;
  padding: var(--spacing-lg);
  min-height: 200px;
}

.loading-message {
  text-align: center;
  color: var(--color-text-secondary);
  padding: var(--spacing-xl) 0;
}

.empty-state {
  text-align: center;
  color: var(--color-text-secondary);
  padding: var(--spacing-xl) 0;
}

.empty-subtitle {
  font-size: 0.875rem;
  margin-top: var(--spacing-sm);
}

.book-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.book-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md);
  background: var(--color-surface-variant);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  transition: background-color var(--transition-fast);
}

.book-item:hover {
  background: var(--color-gray-100);
}

.theme-dark .book-item:hover {
  background: var(--color-gray-800);
}

.book-info {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.book-name {
  font-weight: 500;
  color: var(--color-text);
}

.book-meta {
  font-size: 0.75rem;
  color: var(--color-text-secondary);
}

.remove-button {
  background: var(--color-error-100);
  color: var(--color-error-600);
  border: 1px solid var(--color-error-200);
  border-radius: var(--radius-sm);
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  font-size: 1.125rem;
  line-height: 1;
  transition: all var(--transition-fast);
}

.remove-button:hover {
  background: var(--color-error-200);
  color: var(--color-error-700);
}

.theme-dark .remove-button {
  background: var(--color-error-900);
  color: var(--color-error-400);
  border-color: var(--color-error-800);
}

.theme-dark .remove-button:hover {
  background: var(--color-error-800);
  color: var(--color-error-300);
}

.modal-footer {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: var(--spacing-md);
  padding: var(--spacing-lg);
  border-top: 1px solid var(--color-border);
}

.import-button {
  background: var(--color-primary-500);
  color: white;
  border: none;
  border-radius: var(--radius-md);
  padding: var(--spacing-sm) var(--spacing-lg);
  font-weight: 500;
  cursor: pointer;
  transition: background-color var(--transition-fast);
}

.import-button:hover:not(:disabled) {
  background: var(--color-primary-600);
}

.import-button:disabled,
.cancel-button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.import-progress {
  flex: 1;
  font-size: 0.875rem;
  color: var(--color-text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
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

.delete-modal {
  max-width: 500px;
}

.warning-text {
  color: var(--color-text-secondary);
  font-size: 0.875rem;
  margin-top: var(--spacing-sm);
}

.error-message {
  background: var(--color-error-100);
  color: var(--color-error-700);
  border: 1px solid var(--color-error-200);
  border-radius: var(--radius-md);
  padding: var(--spacing-sm) var(--spacing-md);
  margin: var(--spacing-md) 0;
  font-size: 0.875rem;
}

.theme-dark .error-message {
  background: var(--color-error-900);
  color: var(--color-error-300);
  border-color: var(--color-error-800);
}

.modal-footer {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: var(--spacing-md);
  padding: var(--spacing-lg);
  border-top: 1px solid var(--color-border);
}

.delete-confirm-button {
  background: var(--color-error-500);
  color: white;
  border: none;
  border-radius: var(--radius-md);
  padding: var(--spacing-sm) var(--spacing-lg);
  font-weight: 500;
  cursor: pointer;
  transition: background-color var(--transition-fast);
}

.delete-confirm-button:hover {
  background: var(--color-error-600);
}
</style>