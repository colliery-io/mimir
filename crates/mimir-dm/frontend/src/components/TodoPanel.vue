<template>
  <div class="todo-panel">
    <!-- Toggle button -->
    <button 
      @click="toggleTodos" 
      class="todo-toggle-btn"
      :class="{ 'has-todos': chatStore.hasTodos }"
      :title="chatStore.hasTodos ? `Todo List (${chatStore.todoProgress.completed}/${chatStore.todoProgress.total})` : 'No active todos'"
    >
      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
              d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01"/>
      </svg>
      <span class="ml-2">Todos</span>
      <span v-if="chatStore.hasTodos" class="todo-badge">{{ chatStore.todoProgress.completed }}/{{ chatStore.todoProgress.total }}</span>
    </button>

    <!-- Todo panel -->
    <div v-if="chatStore.todosVisible && chatStore.hasTodos" class="todo-list-container">
      <div class="todo-header">
        <h3 class="todo-title">Task Progress</h3>
        <div class="todo-progress-bar">
          <div class="progress-background">
            <div 
              class="progress-fill"
              :style="{ width: `${chatStore.todoProgress.percentage}%` }"
            ></div>
          </div>
          <span class="progress-text">{{ chatStore.todoProgress.percentage }}% complete</span>
        </div>
      </div>

      <div class="todo-list">
        <div 
          v-for="(todo, index) in chatStore.todos" 
          :key="index"
          class="todo-item"
          :class="`todo-${todo.status}`"
        >
          <div class="todo-icon">
            <svg v-if="todo.status === 'completed'" class="w-4 h-4 text-green-500" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
            </svg>
            <svg v-else-if="todo.status === 'in_progress'" class="w-4 h-4 text-blue-500 animate-spin" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            <svg v-else class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <circle cx="12" cy="12" r="10" stroke-width="2"/>
            </svg>
          </div>
          
          <div class="todo-content">
            <div class="todo-text">
              {{ todo.status === 'in_progress' ? todo.activeForm : todo.content }}
            </div>
          </div>
        </div>
      </div>

      <!-- Current task highlight -->
      <div v-if="chatStore.currentTodo" class="current-task">
        <div class="current-task-label">Currently:</div>
        <div class="current-task-text">{{ chatStore.currentTodo.activeForm }}</div>
      </div>
    </div>

    <!-- Empty state when todos visible but no todos -->
    <div v-if="chatStore.todosVisible && !chatStore.hasTodos" class="empty-todos">
      <div class="empty-icon">
        <svg class="w-8 h-8 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"/>
        </svg>
      </div>
      <p class="empty-text">No active todos</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useChatStore } from '@/stores/chat'

const chatStore = useChatStore()

// Methods
const toggleTodos = () => {
  chatStore.toggleTodosVisibility()
}
</script>

<style scoped>
.todo-panel {
  @apply relative;
}

.todo-toggle-btn {
  @apply flex items-center px-3 py-2 text-sm rounded-lg transition-colors duration-200;
  @apply border border-gray-200 bg-gray-50 text-gray-600;
  @apply hover:bg-gray-100 hover:text-gray-800;
}

.todo-toggle-btn.has-todos {
  @apply bg-blue-50 border-blue-200 text-blue-700;
  @apply hover:bg-blue-100;
}

.todo-badge {
  @apply ml-2 px-2 py-1 text-xs rounded-full;
  @apply bg-blue-100 text-blue-800 font-medium;
}

.todo-list-container {
  @apply absolute top-full left-0 mt-2 w-80 max-w-sm;
  @apply bg-white border border-gray-200 rounded-lg shadow-lg;
  @apply z-50 max-h-96 overflow-y-auto;
}

.todo-header {
  @apply p-4 border-b border-gray-100;
}

.todo-title {
  @apply text-lg font-semibold text-gray-800 mb-3;
}

.todo-progress-bar {
  @apply flex items-center space-x-3;
}

.progress-background {
  @apply flex-1 h-2 bg-gray-200 rounded-full overflow-hidden;
}

.progress-fill {
  @apply h-full bg-blue-500 transition-all duration-300;
}

.progress-text {
  @apply text-xs font-medium text-gray-600 whitespace-nowrap;
}

.todo-list {
  @apply p-2;
}

.todo-item {
  @apply flex items-start space-x-3 p-2 rounded-lg;
  @apply hover:bg-gray-50 transition-colors duration-150;
}

.todo-item.todo-completed {
  @apply opacity-60;
}

.todo-item.todo-in_progress {
  @apply bg-blue-50 border border-blue-100;
}

.todo-icon {
  @apply flex-shrink-0 mt-0.5;
}

.todo-content {
  @apply flex-1 min-w-0;
}

.todo-text {
  @apply text-sm text-gray-700;
  @apply break-words;
}

.todo-item.todo-completed .todo-text {
  @apply text-gray-500 line-through;
}

.todo-item.todo-in_progress .todo-text {
  @apply text-blue-800 font-medium;
}

.current-task {
  @apply p-4 bg-blue-50 border-t border-blue-100;
}

.current-task-label {
  @apply text-xs font-medium text-blue-600 uppercase tracking-wide mb-1;
}

.current-task-text {
  @apply text-sm font-medium text-blue-800;
}

.empty-todos {
  @apply absolute top-full left-0 mt-2 w-80 max-w-sm;
  @apply bg-white border border-gray-200 rounded-lg shadow-lg;
  @apply z-50 p-8 text-center;
}

.empty-icon {
  @apply flex justify-center mb-3;
}

.empty-text {
  @apply text-sm text-gray-500;
}

/* Dark mode support */
@media (prefers-color-scheme: dark) {
  .todo-toggle-btn {
    @apply border-gray-700 bg-gray-800 text-gray-300;
    @apply hover:bg-gray-700 hover:text-gray-100;
  }

  .todo-toggle-btn.has-todos {
    @apply bg-blue-900 border-blue-700 text-blue-200;
    @apply hover:bg-blue-800;
  }

  .todo-badge {
    @apply bg-blue-800 text-blue-100;
  }

  .todo-list-container {
    @apply bg-gray-800 border-gray-700;
  }

  .todo-header {
    @apply border-gray-700;
  }

  .todo-title {
    @apply text-gray-100;
  }

  .progress-background {
    @apply bg-gray-700;
  }

  .progress-text {
    @apply text-gray-400;
  }

  .todo-item {
    @apply hover:bg-gray-700;
  }

  .todo-item.todo-in_progress {
    @apply bg-blue-900 border-blue-700;
  }

  .todo-text {
    @apply text-gray-300;
  }

  .todo-item.todo-completed .todo-text {
    @apply text-gray-500;
  }

  .todo-item.todo-in_progress .todo-text {
    @apply text-blue-200;
  }

  .current-task {
    @apply bg-blue-900 border-blue-700;
  }

  .current-task-label {
    @apply text-blue-300;
  }

  .current-task-text {
    @apply text-blue-100;
  }

  .empty-todos {
    @apply bg-gray-800 border-gray-700;
  }

  .empty-text {
    @apply text-gray-400;
  }
}
</style>