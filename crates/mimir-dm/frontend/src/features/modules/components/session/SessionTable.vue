<template>
  <div class="session-table-container">
    <div class="session-header flex justify-between items-center mb-4">
      <h3 class="text-xl font-bold">Sessions</h3>
      <button @click="$emit('create')" class="btn btn-primary btn-sm">
        Add Session
      </button>
    </div>
    
    <div v-if="sessions.length === 0" class="empty-state text-center py-8">
      <p class="text-gray-500">No sessions yet. Start by creating your first session!</p>
    </div>
    
    <div v-else class="overflow-x-auto">
      <table class="table table-zebra w-full">
        <thead>
          <tr>
            <th>Session</th>
            <th>Date</th>
            <th>Session #</th>
            <th>Status</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="session in sessions" :key="session.id">
            <td>{{ session.name }}</td>
            <td>{{ formatDate(session.scheduled_date || session.actual_date || '') }}</td>
            <td>{{ session.session_number || '-' }}</td>
            <td>
              <span class="badge" :class="getStatusClass(session.status)">
                {{ session.status }}
              </span>
            </td>
            <td>
              <div class="flex gap-2">
                <button 
                  @click="$emit('edit', session)"
                  class="btn btn-ghost btn-xs"
                >
                  Edit
                </button>
                <button 
                  v-if="session.status === 'planned'"
                  @click="$emit('start', session.id)"
                  class="btn btn-success btn-xs"
                >
                  Start
                </button>
                <button 
                  v-if="session.status === 'active'"
                  @click="$emit('complete', session.id)"
                  class="btn btn-info btn-xs"
                >
                  Complete
                </button>
                <button 
                  @click="$emit('delete', session.id)"
                  class="btn btn-error btn-xs"
                >
                  Delete
                </button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Session } from '@/types'

interface Props {
  sessions: Session[]
}

defineProps<Props>()

defineEmits<{
  create: []
  edit: [session: Session]
  start: [id: number | string]
  complete: [id: number | string]
  delete: [id: number | string]
}>()

function formatDate(date: string): string {
  if (!date) return 'Not set'
  try {
    return new Date(date).toLocaleDateString()
  } catch {
    return date
  }
}

function getStatusClass(status: string): string {
  const classes: Record<string, string> = {
    planned: 'badge-info',
    active: 'badge-success',
    completed: 'badge-neutral'
  }
  return classes[status] || 'badge-ghost'
}
</script>