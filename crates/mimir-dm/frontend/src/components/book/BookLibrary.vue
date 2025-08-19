<template>
  <Panel title="Library" variant="surface">
    <template #actions>
      <button 
        @click="handleAddBook" 
        class="add-book-btn"
        title="Add a book archive to your library"
      >
        Add Book
      </button>
    </template>
    
    <div class="library-content">
      <div v-if="isLoadingLibrary" class="loading-message">
        Loading library...
      </div>
      
      <div v-else-if="libraryBooks.length === 0" class="empty-message">
        <p>No books in library</p>
        <p v-if="isDevelopment" class="dev-note">
          Running in development mode
        </p>
      </div>
      
      <div v-else class="book-list">
        <div 
          v-for="book in libraryBooks" 
          :key="book.id"
          :class="['book-item', { active: selectedBook?.id === book.id }]"
          @click="$emit('select', book)"
        >
          <div class="book-info">
            <span class="book-name">{{ book.name }}</span>
            <span class="book-meta">
              <span v-if="book.id === 'test-book'" class="dev-badge">DEV</span>
              <span v-if="book.image_count">{{ book.image_count }} images</span>
            </span>
          </div>
          <button 
            @click.stop="handleRemoveBook(book)"
            class="remove-btn"
            title="Remove from library"
          >
            ×
          </button>
        </div>
      </div>
    </div>
  </Panel>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import Panel from '../layout/Panel.vue'
import type { BookInfo } from '../../types/book'

interface Props {
  libraryBooks: BookInfo[]
  selectedBook: BookInfo | null
  isLoadingLibrary: boolean
  isDevelopment: boolean
}

interface Emits {
  (e: 'select', book: BookInfo): void
  (e: 'add'): void
  (e: 'remove', book: BookInfo): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

function handleAddBook() {
  emit('add')
}

function handleRemoveBook(book: BookInfo) {
  emit('remove', book)
}
</script>

<style scoped>
.library-content {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.add-book-btn {
  padding: 6px 12px;
  background: var(--color-primary, #4a9eff);
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.875rem;
  transition: background-color 0.2s;
}

.add-book-btn:hover {
  background: var(--color-primary-dark, #357abd);
}

.loading-message,
.empty-message {
  padding: var(--spacing-lg, 16px);
  text-align: center;
  color: var(--color-text-secondary, #999);
}

.dev-note {
  margin-top: var(--spacing-sm, 8px);
  font-size: 0.75rem;
  color: var(--color-text-tertiary, #666);
}

.book-list {
  flex: 1;
  overflow-y: auto;
}

.book-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-sm, 8px) var(--spacing-md, 12px);
  cursor: pointer;
  transition: background-color 0.2s;
}

.book-item:hover {
  background: var(--color-surface-hover, rgba(255, 255, 255, 0.05));
}

.book-item.active {
  background: var(--color-primary-alpha, rgba(74, 158, 255, 0.1));
  border-left: 3px solid var(--color-primary, #4a9eff);
}

.book-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.book-name {
  color: var(--color-text, #e0e0e0);
  font-size: 0.9rem;
}

.book-meta {
  font-size: 0.75rem;
  color: var(--color-text-tertiary, #666);
  display: flex;
  align-items: center;
  gap: var(--spacing-xs, 4px);
}

.dev-badge {
  background: var(--color-warning, #ffaa00);
  color: var(--color-background, #0d0d0d);
  padding: 1px 4px;
  border-radius: 3px;
  font-weight: 600;
  text-transform: uppercase;
}

.remove-btn {
  width: 20px;
  height: 20px;
  padding: 0;
  background: transparent;
  color: var(--color-text-secondary, #999);
  border: none;
  border-radius: 3px;
  cursor: pointer;
  font-size: 1.2rem;
  line-height: 1;
  opacity: 0;
  transition: opacity 0.2s, color 0.2s;
}

.book-item:hover .remove-btn {
  opacity: 1;
}

.remove-btn:hover {
  color: var(--color-danger, #ff4444);
  background: var(--color-danger-alpha, rgba(255, 68, 68, 0.1));
}
</style>